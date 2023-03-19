use crate::events::ReadySpawnEvent;
use crate::plugin::DelayerSystemSet;
use bevy::prelude::*;
use bevy::utils::all_tuples;
use std::fmt::Debug;

/// Abstraction over a tuple of [`Spawner`]s.
/// See [`SpewApp::add_spawners`](crate::prelude::SpewApp::add_spawners) for more information.
pub trait Spawners<D = ()> {
    /// Add all spawners to the app. Called internally.
    fn add_to_app(self, app: &mut App);
}

/// Abstraction over a tuple of an enum variant and a spawning function.
/// See [`SpewApp::add_spawners`](crate::prelude::SpewApp::add_spawners) for more information.
pub trait Spawner<D = ()> {
    /// Add the spawner to the app. Called internally.
    fn add_to_app(self, app: &mut App);
}

impl<T, F, D> Spawner<D> for (T, F)
where
    T: Debug + Eq + Send + Sync + 'static,
    F: Fn(&mut World, D) + 'static + Send + Sync,
    D: Send + Sync + 'static,
{
    fn add_to_app(self, app: &mut App) {
        let (object, spawn_function) = self;
        let system = move |world: &mut World| {
            let mut events = world
                .get_resource_mut::<Events<ReadySpawnEvent<T, D>>>()
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
                spawn_function(world, event.data);
            }
        };
        app.add_system(system.after(DelayerSystemSet));
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
