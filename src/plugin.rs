use crate::events::{delay_spawn_events, DelayedSpawnEvent, SpawnEvent};
use crate::spawner::{Spawner, Spawners};
use bevy::prelude::*;

#[derive(Debug)]
pub struct SpewPlugin<T: Eq + Clone + Send + Sync + 'static> {
    spawner_enum_type: std::marker::PhantomData<T>,
}

impl<T: Eq + Clone + Send + Sync + 'static> Default for SpewPlugin<T> {
    fn default() -> Self {
        Self {
            spawner_enum_type: std::marker::PhantomData,
        }
    }
}

impl<T: Eq + Clone + Send + Sync + 'static> Plugin for SpewPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent<T>>()
            .add_event::<DelayedSpawnEvent<T>>()
            .add_system(delay_spawn_events::<T>);
    }

    fn is_unique(&self) -> bool {
        false
    }
}

pub trait SpewApp {
    fn add_spawner<T: Spawner>(&mut self, spawner: T) -> &mut App;
    fn add_spawners<T: Spawners>(&mut self, spawners: T) -> &mut App;
}

impl SpewApp for App {
    fn add_spawner<T: Spawner>(&mut self, spawner: T) -> &mut App {
        spawner.add_to_app(self);
        self
    }
    fn add_spawners<T: Spawners>(&mut self, spawners: T) -> &mut App {
        spawners.add_to_app(self);
        self
    }
}
