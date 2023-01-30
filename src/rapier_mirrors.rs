mod collider;
mod impulse_joint;

use crate::MirrorPlugin;
use bevy::{
    app::PluginGroupBuilder,
    prelude::{Plugin, PluginGroup},
};
use bevy_rapier3d::prelude::{
    AdditionalMassProperties, Collider, ColliderMassProperties, ImpulseJoint,
};

pub use collider::{AdditionalMassPropertiesMirror, ColliderMassPropertiesMirror, ColliderMirror};
pub use impulse_joint::ImpulseJointMirror;

use self::collider::{Compound, CompoundShapeElement};

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
/// - `Collider` (**some collider shape are not implemented yet!**)
/// - `ColliderMassProperties`
/// - `AdditionalMassProperties`
pub struct RapierMirrorsPlugins;

struct AdditionalReflectionsPlugin;
impl Plugin for AdditionalReflectionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Compound>()
            .register_type::<CompoundShapeElement>();
    }
}

impl PluginGroup for RapierMirrorsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AdditionalMassPropertiesMirrorPlugin::new())
            .add(ColliderMassPropertiesMirrorPlugin::new())
            .add(ColliderMirrorPlugin::new())
            .add(ImpulseJointMirrorPlugin::new())
            .add(AdditionalReflectionsPlugin)
    }
}
