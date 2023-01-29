mod collider;
mod impulse_joint;

use crate::MirrorPlugin;
use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, ColliderMassProperties, ImpulseJoint,
};

pub use collider::{AdditionalMassPropertiesMirror, ColliderMassPropertiesMirror, ColliderMirror};
pub use impulse_joint::ImpulseJointMirror;

pub type ImpulseJointMirrorPlugin = MirrorPlugin<ImpulseJoint, ImpulseJointMirror>;
pub type ColliderMirrorPlugin = MirrorPlugin<Collider, ColliderMirror>;
pub type ColliderMassPropertiesMirrorPlugin =
    MirrorPlugin<ColliderMassProperties, ColliderMassPropertiesMirror>;
pub type AdditionalMassPropertiesMirrorPlugin =
    MirrorPlugin<AdditionalMassProperties, AdditionalMassPropertiesMirror>;

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
/// - `Collider` (**warning: some collider shape will panic!**)
/// - `ColliderMassProperties`
/// - `AdditionalMassProperties`
pub struct RapierMirrorsPlugins;

impl PluginGroup for RapierMirrorsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AdditionalMassPropertiesMirrorPlugin::new())
            .add(ColliderMassPropertiesMirrorPlugin::new())
            .add(ColliderMirrorPlugin::new())
            .add(ImpulseJointMirrorPlugin::new())
    }
}
