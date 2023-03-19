use crate::events::SpawnEvent;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::all_tuples;

pub trait Spawners<D> {
    fn add_to_app(self, app: &mut App);
}

pub trait Spawner<D> {
    fn add_to_app(self, app: &mut App);
}

#[derive(Resource, Deref, DerefMut)]
/// See <https://docs.rs/bevy/latest/bevy/ecs/system/struct.SystemState.html#warning>
pub(crate) struct CachedSystemState<
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
>(pub SystemState<EventReader<'static, 'static, SpawnEvent<T, D>>>);

impl<T, F, D> Spawner<D> for (T, F)
where
    T: Eq + Clone + Send + Sync + 'static,
    F: Fn(D, &mut World) + 'static + Send + Sync,
    D: Clone + Send + Sync + 'static,
{
    fn add_to_app(self, app: &mut App) {
        let (object, spawn_function) = self;
        let system = move |world: &mut World| {
            world.resource_scope(|world, mut cached_state: Mut<CachedSystemState<T, D>>| {
                let mut event_reader = cached_state.get_mut(world);
                let data: Vec<_> = event_reader
                    .iter()
                    .filter(|event| event.object == object)
                    .map(|event| event.data.clone())
                    .collect();
                for data in data {
                    spawn_function(data, world);
                }
            });
        };
        app.add_system(system);
    }
}

macro_rules! impl_spawners_tuples {
    ($(($param: ident, $spawners: ident)),*) => {
        impl<$($param, $spawners),*> Spawners<($($param,)*)> for ($($spawners,)*)
        where
            $($spawners: Spawner<$param>),*
        {
            #[allow(non_snake_case, unused_variables)]
            fn add_to_app(self, app: &mut App) {
                let ($($spawners,)*) = self;
                $($spawners.add_to_app(app);)*
            }
        }
    }
}

all_tuples!(impl_spawners_tuples, 0, 15, S, D);
