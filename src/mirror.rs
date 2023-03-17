use std::marker::PhantomData;

use bevy::{prelude::*, reflect::GetTypeRegistration};

/// Mirror `T`
/// If you wish to mirror other components, you need to do the following:
///
/// - Create a `Component` (eg: `ForeignMirror`)
/// - Implement the `Mirror` trait for that component.
/// - Implement `From<&'a Foreign> for ForeignMirror`
/// - Add `MirrorPlugin::<Foreign, ForeignMirror>::new()` to your `app`
///
/// ```rust
/// use bevy_mod_component_mirror::{Mirror, MirrorPlugin};
/// use bevy::prelude::*;
///
/// # mod foreign_crate {
/// #    use super::*; #[derive(Component)]pub struct Foreign; impl Foreign {
/// #   pub fn set_length(&mut self, value: f32) {}
/// #   pub fn length(&self) -> f32 { 0.0 }
/// # }}
/// use foreign_crate::Foreign;
///
/// #[derive(Component, Reflect)]
/// pub struct ForeignMirror {
///   inner: f32,
/// }
///
/// // Foreign → ForeignMirror
/// impl<'a> From<&'a Foreign> for ForeignMirror {
///   fn from(value: &'a Foreign) -> Self {
///     ForeignMirror {
///       inner: value.length(),
///     }
///   }
/// }
/// // ForeignMirror → Foreign
/// impl Mirror<Foreign> for ForeignMirror {
///   fn apply(&self, value: &mut Foreign) {
///     value.set_length(self.inner);
///   }
/// }
///
/// fn main() {
///   let mut app = App::new();
///   app.add_plugin(MirrorPlugin::<Foreign, ForeignMirror>::new());
/// }
///
/// ```
pub trait Mirror<T>: for<'a> From<&'a T> {
    fn apply(&self, val: &mut T);
}
fn reflect_mirror_add<T: Component, U: Mirror<T> + Component>(
    query: Query<(Entity, &T), Added<T>>,
    mut cmds: Commands,
) {
    for (entity, added) in &query {
        cmds.entity(entity).insert(U::from(added));
    }
}
#[allow(clippy::type_complexity)]
fn reflect_mirror_component<T: Component, U: Mirror<T> + Component>(
    mut query: ParamSet<(
        (Query<(Entity, &T), Changed<T>>, Query<&mut U>),
        (Query<&mut T>, Query<(Entity, &U), Changed<U>>),
    )>,
) {
    let (changed, mut to_update) = query.p0();
    for (entity, changed) in &changed {
        if let Ok(mut to_update) = to_update.get_mut(entity) {
            *to_update = changed.into()
        }
    }
    let (mut to_update, changed) = query.p1();
    for (entity, changed) in &changed {
        if let Ok(mut to_update) = to_update.get_mut(entity) {
            changed.apply(&mut to_update)
        }
    }
}

/// Systems added by the [`MirrorPlugin`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum MirrorSystems {
    /// When the mirror component is updated, during [`CoreStage::First`].
    Update,
    /// When mirror components get added to entites with the component they
    /// mirror (if not already present), in [`CoreStage::Last`].
    Add,
}
/// Update each frame [`Component`] `U` with the value of `T` and vis-versa.
///
/// This will add `U` to [`Entity`] with the `T` components, and keep it updated,
/// in [`MirrorSystems::Update`], in [`CoreStage::First`].
///
/// It will also add `T` to the type registry.
///
/// See [`Mirror`] for usage.
///
/// If you only care for rapier components, see [`crate::RapierMirrorsPlugins`].
pub struct MirrorPlugin<T: Component, U: Mirror<T> + Component + GetTypeRegistration>(
    PhantomData<(T, U)>,
);
impl<T: Component, U: Mirror<T> + Component + GetTypeRegistration> MirrorPlugin<T, U> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T: Component, U: Mirror<T> + Component + GetTypeRegistration> Plugin for MirrorPlugin<T, U> {
    fn build(&self, app: &mut App) {
        app.register_type::<U>()
            .add_system(
                reflect_mirror_add::<T, U>
                    .in_set(MirrorSystems::Add)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                reflect_mirror_component::<T, U>
                    .in_set(MirrorSystems::Update)
                    .in_base_set(CoreSet::First),
            );
    }
}
