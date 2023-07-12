use crate::events::{delay_spawn_events, ReadySpawnEvent};
use crate::plugin::SpewSystemSet;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::all_tuples;
use std::fmt::Debug;

/// Abstraction over a tuple of [`Spawner`]s.
/// See [`SpewApp::add_spawners`](crate::prelude::SpewApp::add_spawners) for more information.
pub trait Spawners<Marker>: Send + Sync + 'static {
    /// Add all spawners to the app. Called internally.
    fn add_to_app(self, app: &mut App);
}

/// Abstraction over a tuple of an enum variant and a spawning function.
/// See [`SpewApp::add_spawners`](crate::prelude::SpewApp::add_spawners) for more information.
pub trait Spawner<Marker>: Send + Sync + 'static {
    /// Add the spawner to the app. Called internally.
    fn add_to_app(self, app: &mut App);
}

impl<T, F, Marker> Spawner<Marker> for (T, F)
where
    T: Debug + Eq + Send + Sync + 'static,
    F: SystemParamFunction<Marker>,
    Marker: Send + Sync + 'static,
    F::In: Send + Sync + 'static,
{
    fn add_to_app(self, app: &mut App) {
        let (object, mut spawn_function) = self;
        let system = move |world: &mut World| {
            let mut events = world
                .get_resource_mut::<Events<ReadySpawnEvent<T, F::In>>>()
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
                let mut system_state: SystemState<F::Param> = SystemState::new(world);
                let user_data = event.data;
                let param = system_state.get_mut(world);
                spawn_function.run(user_data, param);
                system_state.apply(world);
            }
        };
        app.add_systems(
            Update,
            system
                .after(delay_spawn_events::<T, F::In>)
                .in_set(SpewSystemSet),
        );
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
