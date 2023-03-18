#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
// #![forbid(missing_docs)]

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::all_tuples;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{SpawnEvent, SpewApp, SpewPlugin};
}

#[derive(Debug)]
pub struct SpewPlugin<T: Eq + Send + Sync + 'static> {
    spawner_enum_type: std::marker::PhantomData<T>,
}

impl<T: Eq + Send + Sync + 'static> Default for SpewPlugin<T> {
    fn default() -> Self {
        Self {
            spawner_enum_type: std::marker::PhantomData,
        }
    }
}

impl<T: Eq + Send + Sync + 'static> Plugin for SpewPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent<T>>();
    }
}

pub struct SpawnEvent<T: Eq + Send + Sync + 'static> {
    pub object: T,
    pub transform: Transform,
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

pub trait Spawners {
    fn add_to_app(self, app: &mut App);
}

pub trait Spawner {
    fn add_to_app(self, app: &mut App);
}

impl<T: Eq + Send + Sync + 'static, F: FnMut(Transform, &mut World) + 'static + Send + Sync> Spawner
    for (T, F)
{
    fn add_to_app(self, app: &mut App) {
        let (object, mut spawn_function) = self;
        let system = move |world: &mut World| {
            let mut event_system_state = SystemState::<EventReader<SpawnEvent<T>>>::new(world);
            let mut events = event_system_state.get_mut(world);
            let transforms: Vec<_> = events
                .iter()
                .filter(|event| event.object == object)
                .map(|event| event.transform)
                .collect();
            for transform in transforms {
                spawn_function(transform, world);
            }
        };
        app.add_system(system);
    }
}

macro_rules! impl_spawners_tuples {
    ($($name: ident),*) => {
        impl<$($name),*> Spawners for ($($name,)*)
            where
                $($name: Spawner),*{
                #[allow(non_snake_case, unused_variables)]
                fn add_to_app(self, app: &mut App) {
                    let ($($name,)*) = self;
                    $($name.add_to_app(app);)*
                }
            }
    }
}

all_tuples!(impl_spawners_tuples, 0, 15, S);
