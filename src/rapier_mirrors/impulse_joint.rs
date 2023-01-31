use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{GenericJoint, ImpulseJoint},
    rapier::prelude::{
        JointAxesMask, JointAxis, JointLimits, JointMotor as RapierJointMotor,
        MotorModel as RapierMotorModel,
    },
};

use crate::Mirror;

/// The spring-like model used for constraints resolution.
#[derive(Reflect, FromReflect, Default)]
#[reflect(Default)]
pub(super) enum MotorModel {
    /// The solved spring-like equation is:
    /// `acceleration = stiffness * (pos - target_pos) + damping * (vel - target_vel)`
    AccelerationBased,
    /// The solved spring-like equation is:
    /// `force = stiffness * (pos - target_pos) + damping * (vel - target_vel)`
    #[default]
    ForceBased,
}
impl From<RapierMotorModel> for MotorModel {
    fn from(value: RapierMotorModel) -> Self {
        match value {
            RapierMotorModel::AccelerationBased => Self::AccelerationBased,
            RapierMotorModel::ForceBased => Self::ForceBased,
        }
    }
}
#[derive(Reflect, FromReflect, Default)]
#[reflect(Default)]
pub(super) struct JointMotor {
    target_vel: Vec3,
    target_pos: Vec3,
    stiffness: Vec3,
    damping: Vec3,
    impulse: Vec3,
    locked: BVec3,
    limit_min: Vec3,
    limit_max: Vec3,
    limit_active: BVec3,
    model: MotorModel,
    #[reflect(ignore)]
    max_force: Vec3,
}
type LimitsFun = fn(&JointLimits<f32>) -> f32;
impl JointMotor {
    fn from_linear(joint: &GenericJoint) -> Option<Self> {
        let mk_vec3 = |f: fn(&RapierJointMotor) -> f32| {
            Some(Vec3 {
                x: f(joint.motor(JointAxis::X)?),
                y: f(joint.motor(JointAxis::Y)?),
                z: f(joint.motor(JointAxis::Z)?),
            })
        };
        let mk_vec3_limit = |f: LimitsFun| Vec3 {
            x: joint.limits(JointAxis::X).map_or(0., f),
            y: joint.limits(JointAxis::Y).map_or(0., f),
            z: joint.limits(JointAxis::Z).map_or(0., f),
        };
        Some(JointMotor {
            target_vel: mk_vec3(|m| m.target_vel)?,
            target_pos: mk_vec3(|m| m.target_pos)?,
            stiffness: mk_vec3(|m| m.stiffness)?,
            damping: mk_vec3(|m| m.damping)?,
            max_force: mk_vec3(|m| m.max_force)?,
            impulse: mk_vec3(|m| m.impulse)?,
            locked: BVec3 {
                x: joint.locked_axes().contains(JointAxis::X.into()),
                y: joint.locked_axes().contains(JointAxis::Y.into()),
                z: joint.locked_axes().contains(JointAxis::Z.into()),
            },
            limit_active: BVec3 {
                x: joint.limits(JointAxis::X).is_some(),
                y: joint.limits(JointAxis::Y).is_some(),
                z: joint.limits(JointAxis::Z).is_some(),
            },
            limit_min: mk_vec3_limit(|l| l.min),
            limit_max: mk_vec3_limit(|l| l.max),
            model: joint.motor(JointAxis::X)?.model.into(),
        })
    }
    fn from_angular(joint: &GenericJoint) -> Option<Self> {
        let mk_vec3 = |f: fn(&RapierJointMotor) -> f32| {
            Some(Vec3 {
                x: f(joint.motor(JointAxis::AngX)?),
                y: f(joint.motor(JointAxis::AngY)?),
                z: f(joint.motor(JointAxis::AngZ)?),
            })
        };
        let mk_vec3_limit = |f: LimitsFun| Vec3 {
            x: joint.limits(JointAxis::AngX).map_or(0., f),
            y: joint.limits(JointAxis::AngY).map_or(0., f),
            z: joint.limits(JointAxis::AngZ).map_or(0., f),
        };
        Some(JointMotor {
            target_vel: mk_vec3(|m| m.target_vel)?,
            target_pos: mk_vec3(|m| m.target_pos)?,
            stiffness: mk_vec3(|m| m.stiffness)?,
            damping: mk_vec3(|m| m.damping)?,
            max_force: mk_vec3(|m| m.max_force)?,
            impulse: mk_vec3(|m| m.impulse)?,
            locked: BVec3 {
                x: joint.locked_axes().contains(JointAxis::AngX.into()),
                y: joint.locked_axes().contains(JointAxis::AngY.into()),
                z: joint.locked_axes().contains(JointAxis::AngZ.into()),
            },
            limit_active: BVec3 {
                x: joint.limits(JointAxis::AngX).is_some(),
                y: joint.limits(JointAxis::AngY).is_some(),
                z: joint.limits(JointAxis::AngZ).is_some(),
            },
            limit_min: mk_vec3_limit(|l| l.min),
            limit_max: mk_vec3_limit(|l| l.max),
            model: joint.motor(JointAxis::AngX)?.model.into(),
        })
    }
}

#[derive(Reflect, Component)]
pub struct ImpulseJointMirror {
    parent: Entity,
    angular: Option<JointMotor>,
    linear: Option<JointMotor>,
    contacts: bool,
}
impl<'a> From<&'a ImpulseJoint> for ImpulseJointMirror {
    fn from(value: &'a ImpulseJoint) -> Self {
        ImpulseJointMirror {
            parent: value.parent,
            angular: JointMotor::from_angular(&value.data),
            linear: JointMotor::from_linear(&value.data),
            contacts: value.data.contacts_enabled(),
        }
    }
}
fn component_of(joint: JointAxis) -> fn(Vec3) -> f32 {
    use JointAxis::*;
    match joint {
        X | AngX => |v| v.x,
        Y | AngY => |v| v.y,
        Z | AngZ => |v| v.z,
    }
}
fn component_of_b(joint: JointAxis) -> fn(BVec3) -> bool {
    use JointAxis::*;
    match joint {
        X | AngX => |v| v.x,
        Y | AngY => |v| v.y,
        Z | AngZ => |v| v.z,
    }
}
impl Mirror<ImpulseJoint> for ImpulseJointMirror {
    fn apply(&self, val: &mut ImpulseJoint) {
        use JointAxis::*;
        val.parent = self.parent;
        for (i, joint, source) in [
            (0, X, &self.linear),
            (1, Y, &self.linear),
            (2, Z, &self.linear),
            (3, AngX, &self.angular),
            (4, AngY, &self.angular),
            (5, AngZ, &self.angular),
        ] {
            let Some(source) = source else { continue };
            let c = component_of(joint);
            let b = component_of_b(joint);
            val.data.set_motor(
                joint,
                c(source.target_pos),
                c(source.target_vel),
                c(source.stiffness),
                c(source.damping),
            );
            val.data.raw.limit_axes &= !JointAxesMask::from(joint);
            if b(source.limit_active) {
                val.data.raw.limit_axes |= joint.into();
            }
            if b(source.limit_active) {
                let (min, max) = (c(source.limit_min), c(source.limit_max));
                val.data.raw.limits[i] = JointLimits {
                    min,
                    max,
                    impulse: c(source.impulse),
                };
            }
            val.data.set_motor_max_force(joint, c(source.max_force));

            val.data.raw.locked_axes &= !JointAxesMask::from(joint);
            if b(source.locked) {
                val.data.raw.locked_axes |= joint.into();
            }
        }
    }
}
