use crate::events::SpawnEvent;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::all_tuples;

pub trait Spawners {
    fn add_to_app(self, app: &mut App);
}

pub trait Spawner {
    fn add_to_app(self, app: &mut App);
}

#[derive(Resource, Deref, DerefMut)]
/// See <https://docs.rs/bevy/latest/bevy/ecs/system/struct.SystemState.html#warning>
pub(crate) struct CachedSystemState<T: Eq + Clone + Send + Sync + 'static>(
    pub SystemState<EventReader<'static, 'static, SpawnEvent<T>>>,
);

impl<
        T: Eq + Clone + Send + Sync + 'static,
        F: FnMut(Transform, &mut World) + 'static + Send + Sync,
    > Spawner for (T, F)
{
    fn add_to_app(self, app: &mut App) {
        let (object, mut spawn_function) = self;
        let system = move |world: &mut World| {
            world.resource_scope(|world, mut cached_state: Mut<CachedSystemState<T>>| {
                let mut event_reader = cached_state.get_mut(world);
                let transforms: Vec<_> = event_reader
                    .iter()
                    .filter(|event| event.object == object)
                    .map(|event| event.transform)
                    .collect();
                for transform in transforms {
                    spawn_function(transform, world);
                }
            });
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
