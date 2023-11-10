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
bevy_mod_component_mirror = "0.12.0"
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
- `Collider` (**some collider shape are not implemented yet!**)

### Implement your own mirrors

If you wish to mirror other components, you need to do the following:

- Create a `Component` (eg: `ForeignMirror`)
- Implement the `Mirror` trait for that component.
- Implement `From<&'a Foreign> for ForeignMirror`
- Add `MirrorPlugin::<Foreign, ForeignMirror>::new()` to your `app`

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
  app.add_plugins(MirrorPlugin::<Foreign, ForeignMirror>::new());
}

```


### Features

If you don't need the definitions for the rapier components
but still wish to use the mirror plugin,
you can disable the rapier components with:

```toml
[dependencies]
bevy_mod_component_mirror = { version = "0.12.0", default-features = false }
```

## Version matrix


| bevy | bevy_rapier3d | bevy_mod_component_mirror |
|------|---------------|---------------------------|
| 0.12 | 0.23.0 | 0.11.0, 0.12.0 |
| 0.11 | 0.22.0 | 0.10.0 |
| 0.10 | 0.21.0 | 0.9 |
| 0.9  | 0.20.0 | 0.7 |

## Change log

* `0.12`: **BREAKING**: Removes Components that Bevy Rapier already reflects (ColliderMassProperties, AdditionalMassProperties, MassProperties)
* `0.11`: **BREAKING**: Bump to bevy 0.12 & rapier 0.23
* `0.10`: **BREAKING**: Bump to bevy 0.11 & rapier 0.22 (Thanks Naomijub on GitHub, See #3)
* `0.9`: Fix a compilation error which source is currently unknown

## Development

Consider moving `.git/hooks/pre-commit.sample` to `.git/hooks/pre-commit.sample`, and
adding the following lines **before** the last `exec` one:

```sh
if ! make pre-hook ; then
  echo Error: Some pre-commit checks did not pass!
  exit 1
fi
```

## License

Copyright © 2022 Nicola Papale

This software is licensed under either MIT or Apache 2.0 at your leisure. See
licenses directory for details.

[`bevy_rapier`]: https://lib.rs/crates/bevy_rapier3d
[`bevy-inspector-egui`]: https://lib.rs/crates/bevy-inspector-egui

