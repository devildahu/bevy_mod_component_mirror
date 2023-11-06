use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{
        AdditionalMassProperties, Collider, ColliderMassProperties,
        MassProperties as RapierMassProperties,
    },
    rapier::prelude::{RoundShape, Shape as RapierShape, SharedShape},
    rapier::{
        parry::shape,
        prelude::{Isometry, TypedShape},
    },
};

use crate::Mirror;

#[derive(Clone)]
pub struct ShapeHolder(SharedShape);
impl Default for ShapeHolder {
    fn default() -> Self {
        Self(SharedShape::ball(1.0))
    }
}

#[derive(Clone, Reflect, Default, Component)]
#[reflect(Default)]
pub(super) struct CompoundShapeElement {
    offset: Vec3,
    rotation: Quat,
    shape: ColliderMirror,
}

#[derive(Clone, Reflect, Component)]
#[reflect(Default)]
pub struct Compound(Vec<CompoundShapeElement>);
impl Default for Compound {
    fn default() -> Self {
        Self(vec![Default::default()])
    }
}
impl Compound {
    fn into_rapier(&self) -> Vec<(Isometry<f32>, SharedShape)> {
        self.0
            .iter()
            .map(|mirror| {
                (
                    Isometry::from_parts(mirror.offset.into(), mirror.rotation.into()),
                    (&mirror.shape).into(),
                )
            })
            .collect()
    }
    fn from_rapier(elems: &[(Isometry<f32>, SharedShape)]) -> Self {
        Self(
            elems
                .iter()
                .map(|rapier| CompoundShapeElement {
                    offset: rapier.0.translation.into(),
                    rotation: rapier.0.rotation.into(),
                    shape: (&rapier.1).into(),
                })
                .collect(),
        )
    }
}

#[derive(Clone, Reflect)]
#[reflect(Default)]
#[reflect(from_reflect = false)]
pub enum Shape {
    Ball {
        radius: f32,
    },
    #[reflect(default)]
    Cuboid {
        half_extents: Vec3,
    },
    Capsule {
        a: Vec3,
        b: Vec3,
        radius: f32,
    },
    Segment {
        a: Vec3,
        b: Vec3,
    },
    Triangle {
        a: Vec3,
        b: Vec3,
        c: Vec3,
    },
    // TriMesh {},
    // Polyline {},
    // HalfSpace { normal: Vec3 },
    // HeightField {},
    Compound(Compound),
    // ConvexPolyhedron {},
    Cylinder {
        half_height: f32,
        radius: f32,
    },
    Cone {
        half_height: f32,
        radius: f32,
    },

    UnimplementedYet(#[reflect(ignore)] ShapeHolder),
}

impl FromReflect for Shape {
    fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
        let reflected = reflect.downcast_ref::<Self>()?;
        Some(reflected.clone())
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Cuboid {
            half_extents: Vec3::ONE,
        }
    }
}
#[derive(Clone, Reflect, Component, Default)]
#[reflect(Default)]
pub struct ColliderMirror {
    pub shape: Shape,
    pub shape_rounded: Option<f32>,
}
impl<'a> From<&'a SharedShape> for ColliderMirror {
    fn from(value: &'a SharedShape) -> Self {
        use TypedShape as R;
        let shape_rounded = match value.as_typed_shape() {
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
        Self {
            shape: value.into(),
            shape_rounded,
        }
    }
}
impl<'a> From<&'a SharedShape> for Shape {
    fn from(value: &'a SharedShape) -> Self {
        use TypedShape as R;
        match value.as_typed_shape() {
            R::Ball(v) => Self::Ball { radius: v.radius },
            R::Cuboid(v) => Self::Cuboid {
                half_extents: v.half_extents.into(),
            },
            R::Capsule(v) => Self::Capsule {
                a: v.segment.a.into(),
                b: v.segment.b.into(),
                radius: v.radius,
            },
            R::Segment(v) => Self::Segment {
                a: v.a.into(),
                b: v.b.into(),
            },
            R::Triangle(v) => Self::Triangle {
                a: v.a.into(),
                b: v.b.into(),
                c: v.c.into(),
            },
            R::Compound(v) => Self::Compound(Compound::from_rapier(v.shapes())),
            R::TriMesh(_)
            | R::Polyline(_)
            | R::HalfSpace(_)
            | R::HeightField(_)
            | R::ConvexPolyhedron(_)
            | R::RoundConvexPolyhedron(_)
            | R::Custom(_) => Self::UnimplementedYet(ShapeHolder(value.clone())),

            R::Cylinder(v) => Self::Cylinder {
                half_height: v.half_height,
                radius: v.radius,
            },
            R::Cone(v) => Self::Cone {
                half_height: v.half_height,
                radius: v.radius,
            },
            R::RoundCuboid(v) => Self::Cuboid {
                half_extents: v.inner_shape.half_extents.into(),
            },
            R::RoundTriangle(v) => Self::Triangle {
                a: v.inner_shape.a.into(),
                b: v.inner_shape.b.into(),
                c: v.inner_shape.c.into(),
            },
            R::RoundCylinder(v) => Self::Cylinder {
                half_height: v.inner_shape.half_height,
                radius: v.inner_shape.radius,
            },
            R::RoundCone(v) => Self::Cone {
                half_height: v.inner_shape.half_height,
                radius: v.inner_shape.radius,
            },
        }
    }
}

impl<'a> From<&'a Collider> for ColliderMirror {
    fn from(value: &'a Collider) -> Self {
        Self::from(&value.raw)
    }
}
impl<'a> From<&'a ColliderMirror> for SharedShape {
    fn from(value: &'a ColliderMirror) -> Self {
        use Shape as S;
        macro_rules! set_shape {
            (@shape $shape:ident ( $($args:expr),* )) => {
                shape::$shape::new($($args .into()),*)
            };
            (round $shape:ident $args:tt) => {{
                let shape: Box<dyn RapierShape> = match value.shape_rounded {
                    Some(radius) => Box::new(RoundShape {
                        inner_shape:  set_shape!(@shape $shape $args),
                        border_radius: radius,
                    }),
                    None => Box::new(set_shape!(@shape $shape $args)),
                };
                Self(shape.into()).into()
            }};
            ($shape:ident $args:tt) => {{
                let shape: Box<dyn RapierShape> = Box::new(set_shape!(@shape $shape $args));
                Self(shape.into()).into()
            }}
        }
        match value.shape {
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
            S::Compound(ref elems) => set_shape!(Compound(elems.into_rapier())),
            S::UnimplementedYet(ref shape) => shape.0.clone(),
        }
    }
}
impl Mirror<Collider> for ColliderMirror {
    fn apply(&self, val: &mut Collider) {
        val.raw = self.into();
    }
}
#[derive(Clone, Reflect, Debug)]
pub struct MassProps {
    pub local_center_of_mass: Vec3,
    pub mass: f32,
    pub principal_inertia: Vec3,
    pub inertia_local_frame: Quat,
}

impl MassProps {
    const fn into_rapier(&self) -> RapierMassProperties {
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
        Self {
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
        match value {
            Rapier::Mass(mass) => Self::Mass(*mass),
            Rapier::MassProperties(props) => Self::Props(props.into()),
        }
    }
}
impl<'a> From<&'a ColliderMassProperties> for ColliderMassPropertiesMirror {
    fn from(value: &'a ColliderMassProperties) -> Self {
        use ColliderMassProperties as Rapier;
        match value {
            Rapier::Density(value) => Self::Density(*value),
            Rapier::Mass(value) => Self::Mass(*value),
            Rapier::MassProperties(value) => Self::Props(value.into()),
        }
    }
}
impl Mirror<AdditionalMassProperties> for AdditionalMassPropertiesMirror {
    fn apply(&self, val: &mut AdditionalMassProperties) {
        use AdditionalMassProperties as Rapier;
        *val = match self {
            Self::Mass(value) => Rapier::Mass(*value),
            Self::Props(value) => Rapier::MassProperties(value.into_rapier()),
        };
    }
}
impl Mirror<ColliderMassProperties> for ColliderMassPropertiesMirror {
    fn apply(&self, val: &mut ColliderMassProperties) {
        use ColliderMassProperties as Rapier;
        *val = match self {
            Self::Density(value) => Rapier::Density(*value),
            Self::Mass(value) => Rapier::Mass(*value),
            Self::Props(value) => Rapier::MassProperties(value.into_rapier()),
        };
    }
}
