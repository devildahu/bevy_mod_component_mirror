#![doc = include_str!("../Readme.md")]
#![allow(clippy::wrong_self_convention)]

mod mirror;

/// Rapier mirror definitions.
#[cfg(feature = "rapier")]
pub mod rapier_mirrors;

#[cfg(feature = "rapier")]
pub use rapier_mirrors::RapierMirrorsPlugins;

pub use mirror::{Mirror, MirrorPlugin, MirrorSystems};
