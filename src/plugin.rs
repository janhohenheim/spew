use crate::events::{delay_spawn_events, DelayedSpawnEvent, SpawnEvent};
use crate::spawner::{Spawner, Spawners};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

#[derive(Debug)]
pub struct SpewPlugin<T, D>
where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    _spawner_enum_type: std::marker::PhantomData<T>,
    _data_type: std::marker::PhantomData<D>,
}

impl<T, D> Default for SpewPlugin<T, D>
where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            _spawner_enum_type: std::marker::PhantomData,
            _data_type: std::marker::PhantomData,
        }
    }
}

impl<T, D> Plugin for SpewPlugin<T, D>
where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent<T, D>>()
            .add_event::<DelayedSpawnEvent<T, D>>()
            .add_system(delay_spawn_events::<T, D>);
    }

    fn is_unique(&self) -> bool {
        false
    }
}

pub trait SpewApp {
    fn add_spawner<D, T: Spawner<D>>(&mut self, spawner: T) -> &mut App;
    fn add_spawners<D, T: Spawners<D>>(&mut self, spawners: T) -> &mut App;
}

impl SpewApp for App {
    fn add_spawner<D, T: Spawner<D>>(&mut self, spawner: T) -> &mut App {
        spawner.add_to_app(self);
        self
    }
    fn add_spawners<D, T: Spawners<D>>(&mut self, spawners: T) -> &mut App {
        spawners.add_to_app(self);
        self
    }
}
