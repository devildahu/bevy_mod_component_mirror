# Bevy Component Mirrors

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![Latest version](https://img.shields.io/crates/v/bevy_mod_component_mirror.svg)](https://crates.io/crates/bevy_mod_component_mirror)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Documentation](https://docs.rs/bevy_mod_component_mirror/badge.svg)](https://docs.rs/bevy_mod_component_mirror/)

A third party crate to mirror `Component` values.

By default, it also provides a set of bevy `Component`s 
mirroring the values of [`bevy_rapier`] `Component`s.
(Currently only `bevy_rapier3d`, PRs welcome!)
Since some of [`bevy_rapier`] `Component`s do not implement `Reflect`,
they may be harder to work with.
This crate is especially useful with [`bevy-inspector-egui`],
it will allow you to edit rapier values at run time,
so you don't have to restart your game repetitively
to find the right physics parameters.

## Usage

1. Add this crate as a dependency to your `Cargo.toml`.

```toml
[dependencies]
bevy_mod_component_mirror = "<current version>"
```

2. Add `RapierMirrorsPlugins` to your app

```rust
use bevy_mod_component_mirror::RapierMirrorsPlugins;

# fn main() {
# let mut app = bevy::prelude::App::new();
app
  // Notice  v the plural
  .add_plugins(RapierMirrorsPlugins);
# }
```

That's it! Now every `Entity` with the following rapier (**3d**) components
will automatically have an equivalent `XyzMirror` component that automatically
syncs its value with it.

- `ImpulseJoint`
- `Collider` (**warning: some collider shape will panic!**)
- `ColliderMassProperties`
- `AdditionalMassProperties`

### Implement your own mirrors

If you wish to mirror other components, you need to do the following:

- Create a `Component` (eg: `ForeignMirror`)
- Implement the `Mirror` trait for that component.
- Implement `From<&'a Foreign> for ForeignMirror`
- Add `MirrorPlugin::<ForeignMirror, Foreign>::new()` to your `app`

```rust
use bevy_mod_component_mirror::{Mirror, MirrorPlugin};
use bevy::prelude::*;

# mod foreign_crate {
#   use super::*;
#   #[derive(Component)]
#   pub struct Foreign; impl Foreign {
#   pub fn set_length(&mut self, value: f32) {}
#   pub fn length(&self) -> f32 { 0.0 }
# }}
use foreign_crate::Foreign;

// Component: required because you want it to be a component
// Reflect: this let `MirrorPlugin` register the `Mirror` type itself
#[derive(Component, Reflect)]
pub struct ForeignMirror {
  inner: f32,
}

// Foreign → ForeignMirror
impl<'a> From<&'a Foreign> for ForeignMirror {
  fn from(value: &'a Foreign) -> Self {
    ForeignMirror {
      inner: value.length(),
    }
  }
}
// ForeignMirror → Foreign
impl Mirror<Foreign> for ForeignMirror {
  fn apply(&self, value: &mut Foreign) {
    value.set_length(self.inner);
  }
}

fn main() {
  let mut app = App::new();
  app.add_plugin(MirrorPlugin::<Foreign, ForeignMirror>::new());
}

```


### Features

If you don't need the definitions for the rapier components
but still wish to use the mirror plugin,
you can disable the rapier components with:

```toml
[dependencies]
bevy_mod_component_mirror = { version = "<fill in>", default-features = false }
```

## Version matrix


| bevy | bevy_rapier3d | bevy_mod_component_mirror |
|------|---------------|---------------------------|
| 0.9  |        0.20.0 |                     0.2.0 |

## License

Copyright © 2022 Nicola Papale

This software is licensed under either MIT or Apache 2.0 at your leisure. See
licenses directory for details.

[`bevy_rapier`]: https://lib.rs/crates/bevy_rapier3d
[`bevy-inspector-egui`]: https://lib.rs/crates/bevy-inspector-egui

