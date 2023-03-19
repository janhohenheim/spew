#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]

mod events;
mod plugin;
mod spawner;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{
        events::{DelayedSpawnEvent, SpawnEvent},
        plugin::SpewApp,
        plugin::SpewPlugin,
    };
}
