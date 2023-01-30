use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{
        AdditionalMassProperties, Collider, ColliderMassProperties,
        MassProperties as RapierMassProperties,
    },
    rapier::prelude::{RoundShape, Shape as RapierShape, SharedShape},
    rapier::{parry::shape, prelude::TypedShape},
};

use crate::Mirror;

#[derive(Clone)]
pub struct ShapeHolder(SharedShape);
impl Default for ShapeHolder {
    fn default() -> Self {
        ShapeHolder(SharedShape::ball(1.0))
    }
}

#[derive(Clone, Reflect)]
pub enum Shape {
    Ball { radius: f32 },
    Cuboid { half_extents: Vec3 },
    Capsule { a: Vec3, b: Vec3, radius: f32 },
    Segment { a: Vec3, b: Vec3 },
    Triangle { a: Vec3, b: Vec3, c: Vec3 },
    // TriMesh {},
    // Polyline {},
    // HalfSpace { normal: Vec3 },
    // HeightField {},
    // Compound( ),
    // ConvexPolyhedron {},
    Cylinder { half_height: f32, radius: f32 },
    Cone { half_height: f32, radius: f32 },
    UnimplementedYet(#[reflect(ignore)] ShapeHolder),
}
#[derive(Clone, Reflect, Component)]
pub struct ColliderMirror {
    pub shape: Shape,
    pub shape_rounded: Option<f32>,
}
impl<'a> From<&'a SharedShape> for Shape {
    fn from(value: &'a SharedShape) -> Self {
        use TypedShape as R;
        match value.as_typed_shape() {
            R::Ball(v) => Shape::Ball { radius: v.radius },
            R::Cuboid(v) => Shape::Cuboid {
                half_extents: v.half_extents.into(),
            },
            R::Capsule(v) => Shape::Capsule {
                a: v.segment.a.into(),
                b: v.segment.b.into(),
                radius: v.radius,
            },
            R::Segment(v) => Shape::Segment {
                a: v.a.into(),
                b: v.b.into(),
            },
            R::Triangle(v) => Shape::Triangle {
                a: v.a.into(),
                b: v.b.into(),
                c: v.c.into(),
            },
            R::TriMesh(_)
            | R::Polyline(_)
            | R::HalfSpace(_)
            | R::HeightField(_)
            | R::Compound(_)
            | R::ConvexPolyhedron(_)
            | R::RoundConvexPolyhedron(_)
            | R::Custom(_) => Shape::UnimplementedYet(ShapeHolder(value.clone())),

            R::Cylinder(v) => Shape::Cylinder {
                half_height: v.half_height,
                radius: v.radius,
            },
            R::Cone(v) => Shape::Cone {
                half_height: v.half_height,
                radius: v.radius,
            },
            R::RoundCuboid(v) => Shape::Cuboid {
                half_extents: v.inner_shape.half_extents.into(),
            },
            R::RoundTriangle(v) => Shape::Triangle {
                a: v.inner_shape.a.into(),
                b: v.inner_shape.b.into(),
                c: v.inner_shape.c.into(),
            },
            R::RoundCylinder(v) => Shape::Cylinder {
                half_height: v.inner_shape.half_height,
                radius: v.inner_shape.radius,
            },
            R::RoundCone(v) => Shape::Cone {
                half_height: v.inner_shape.half_height,
                radius: v.inner_shape.radius,
            },
        }
    }
}

impl<'a> From<&'a Collider> for ColliderMirror {
    fn from(value: &'a Collider) -> Self {
        use TypedShape as R;
        let shape_rounded = match value.raw.as_typed_shape() {
            R::Ball(_)
            | R::Cuboid(_)
            | R::Capsule(_)
            | R::Segment(_)
            | R::Triangle(_)
            | R::TriMesh(_)
            | R::Polyline(_)
            | R::HalfSpace(_)
            | R::HeightField(_)
            | R::Compound(_)
            | R::ConvexPolyhedron(_)
            | R::Cylinder(_)
            | R::Cone(_)
            | R::Custom(_) => None,
            R::RoundCuboid(v) => Some(v.border_radius),
            R::RoundTriangle(v) => Some(v.border_radius),
            R::RoundCylinder(v) => Some(v.border_radius),
            R::RoundCone(v) => Some(v.border_radius),
            R::RoundConvexPolyhedron(v) => Some(v.border_radius),
        };
        ColliderMirror {
            shape: Shape::from(&value.raw),
            shape_rounded,
        }
    }
}
impl Mirror<Collider> for ColliderMirror {
    fn apply(&self, val: &mut Collider) {
        use Shape as S;
        macro_rules! set_shape {
            (@shape $shape:ident ( $($args:expr),* )) => {
                shape::$shape::new($($args .into()),*)
            };
            (round $shape:ident $args:tt) => {{
                let shape: Box<dyn RapierShape> = match self.shape_rounded {
                    Some(radius) => Box::new(RoundShape {
                        inner_shape:  set_shape!(@shape $shape $args),
                        border_radius: radius,
                    }),
                    None => Box::new(set_shape!(@shape $shape $args)),
                };
                val.raw = SharedShape(shape.into()).into();
            }};
            ($shape:ident $args:tt) => {{
                let shape: Box<dyn RapierShape> = Box::new(set_shape!(@shape $shape $args));
                val.raw = SharedShape(shape.into()).into();
            }}
        }
        match self.shape {
            S::Ball { radius } => set_shape!(Ball(radius)),
            S::Cuboid { half_extents } => set_shape!(round Cuboid(half_extents)),
            S::Capsule { a, b, radius } => set_shape!(Capsule(a, b, radius)),
            S::Segment { a, b } => set_shape!(Segment(a, b)),
            S::Triangle { a, b, c } => set_shape!(round Triangle(a, b, c)),
            // S::HalfSpace { normal } => set_shape!(HalfSpace(normal)),
            S::Cylinder {
                half_height,
                radius,
            } => set_shape!(round Cylinder(half_height, radius)),
            S::Cone {
                half_height,
                radius,
            } => set_shape!(round Cone(half_height, radius)),
            S::UnimplementedYet(ref shape) => val.raw = shape.0.clone(),
        }
    }
}
#[derive(Clone, Reflect, Debug, FromReflect)]
pub struct MassProps {
    pub local_center_of_mass: Vec3,
    pub mass: f32,
    pub principal_inertia: Vec3,
    pub inertia_local_frame: Quat,
}
impl MassProps {
    fn into_rapier(&self) -> RapierMassProperties {
        RapierMassProperties {
            local_center_of_mass: self.local_center_of_mass,
            mass: self.mass,
            principal_inertia_local_frame: self.inertia_local_frame,
            principal_inertia: self.principal_inertia,
        }
    }
}
#[derive(Clone, Reflect, Debug, Component)]
pub enum AdditionalMassPropertiesMirror {
    Mass(f32),
    Props(MassProps),
}

#[derive(Clone, Reflect, Debug, Component)]
pub enum ColliderMassPropertiesMirror {
    Density(f32),
    Mass(f32),
    Props(MassProps),
}

impl<'a> From<&'a RapierMassProperties> for MassProps {
    fn from(value: &'a RapierMassProperties) -> Self {
        MassProps {
            local_center_of_mass: value.local_center_of_mass,
            mass: value.mass,
            principal_inertia: value.principal_inertia,
            inertia_local_frame: value.principal_inertia_local_frame,
        }
    }
}
impl<'a> From<&'a AdditionalMassProperties> for AdditionalMassPropertiesMirror {
    fn from(value: &'a AdditionalMassProperties) -> Self {
        use AdditionalMassProperties as Rapier;
        use AdditionalMassPropertiesMirror as Mirror;
        match value {
            Rapier::Mass(mass) => Mirror::Mass(*mass),
            Rapier::MassProperties(props) => Mirror::Props(props.into()),
        }
    }
}
impl<'a> From<&'a ColliderMassProperties> for ColliderMassPropertiesMirror {
    fn from(value: &'a ColliderMassProperties) -> Self {
        use ColliderMassProperties as Rapier;
        use ColliderMassPropertiesMirror as Mirror;
        match value {
            Rapier::Density(value) => Mirror::Density(*value),
            Rapier::Mass(value) => Mirror::Mass(*value),
            Rapier::MassProperties(value) => Mirror::Props(value.into()),
        }
    }
}
impl Mirror<AdditionalMassProperties> for AdditionalMassPropertiesMirror {
    fn apply(&self, val: &mut AdditionalMassProperties) {
        use AdditionalMassProperties as Rapier;
        use AdditionalMassPropertiesMirror as Mirror;
        *val = match self {
            Mirror::Mass(value) => Rapier::Mass(*value),
            Mirror::Props(value) => Rapier::MassProperties(value.into_rapier()),
        };
    }
}
impl Mirror<ColliderMassProperties> for ColliderMassPropertiesMirror {
    fn apply(&self, val: &mut ColliderMassProperties) {
        use ColliderMassProperties as Rapier;
        use ColliderMassPropertiesMirror as Mirror;
        *val = match self {
            Mirror::Density(value) => Rapier::Density(*value),
            Mirror::Mass(value) => Rapier::Mass(*value),
            Mirror::Props(value) => Rapier::MassProperties(value.into_rapier()),
        };
    }
}
