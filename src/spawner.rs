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
