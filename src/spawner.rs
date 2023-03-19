use crate::events::SpawnEvent;
use bevy::prelude::*;
use bevy::utils::all_tuples;
use std::fmt::Debug;

pub trait Spawners<D> {
    fn add_to_app(self, app: &mut App);
}

pub trait Spawner<D> {
    fn add_to_app(self, app: &mut App);
}

impl<T, F, D> Spawner<D> for (T, F)
where
    T: Debug + Eq + Send + Sync + 'static,
    F: Fn(D, &mut World) + 'static + Send + Sync,
    D: Send + Sync + 'static,
{
    fn add_to_app(self, app: &mut App) {
        let (object, spawn_function) = self;
        let system = move |world: &mut World| {
            let mut events = world
                .get_resource_mut::<Events<SpawnEvent<T, D>>>()
                .unwrap();
            let mut handled_events = Vec::new();
            let mut unhandled_events = Vec::new();

            for event in events.drain() {
                if event.object == object {
                    handled_events.push(event);
                } else {
                    unhandled_events.push(event);
                }
            }

            for event in unhandled_events {
                events.send(event);
            }

            for event in handled_events {
                spawn_function(event.data, world);
            }
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
