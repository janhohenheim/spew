#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
// #![forbid(missing_docs)]

mod events;
mod plugin;
mod spawner;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{events::SpawnEvent, plugin::SpewApp, plugin::SpewPlugin};
}
