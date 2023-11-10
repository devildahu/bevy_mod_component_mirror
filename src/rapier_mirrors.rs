mod collider;
mod impulse_joint;

use crate::MirrorPlugin;
use bevy::prelude::Plugin;
use bevy_rapier3d::prelude::{Collider, ImpulseJoint};

pub use collider::ColliderMirror;
pub use impulse_joint::ImpulseJointMirror;

use self::{
    collider::{Compound, CompoundShapeElement},
    impulse_joint::{Frame, JointMotor, MotorModel},
};

pub type ImpulseJointMirrorPlugin = MirrorPlugin<ImpulseJoint, ImpulseJointMirror>;
pub type ColliderMirrorPlugin = MirrorPlugin<Collider, ColliderMirror>;

/// Add components mirroring non-reflect rapier components.
///
/// ```rust
/// use bevy_mod_component_mirror::RapierMirrorsPlugins;
/// # fn main() {
/// # let mut app = bevy::prelude::App::new();
/// app
///   // Notice  v the plural
///   .add_plugins(RapierMirrorsPlugins);
/// # }
/// ```
///
/// That's it! Now every `Entity` with the following rapier (**3d**) components
/// will automatically have an equivalent `XyzMirror` component that automatically
/// syncs its value with it.
///
/// - `ImpulseJoint`
/// - `Collider` (**some collider shape are not implemented yet!**)
pub struct RapierMirrorsPlugins;

struct AdditionalReflectionsPlugin;
impl Plugin for AdditionalReflectionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Compound>()
            .register_type::<JointMotor>()
            .register_type::<MotorModel>()
            .register_type::<Frame>()
            .register_type::<CompoundShapeElement>();
    }
}

impl Plugin for RapierMirrorsPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            ColliderMirrorPlugin::new(),
            ImpulseJointMirrorPlugin::new(),
            AdditionalReflectionsPlugin,
        ));
    }
}
