use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Object {
    Cube,
}

#[derive(Clone)]
struct SpawnData {
    transform: Transform,
    name: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Object, SpawnData>::default())
        .add_plugin(SpewPlugin::<Object, (Transform, String)>::default())
        .add_spawners((
            (Object::Cube, spawn_with_struct),
            (Object::Cube, spawn_with_tuple),
        ))
        .add_system(spawn_something_with_struct.on_startup())
        .add_system(spawn_something_with_tuple.on_startup())
        .run();
}

fn spawn_something_with_struct(mut spawn_events: EventWriter<SpawnEvent<Object, SpawnData>>) {
    spawn_events.send(SpawnEvent {
        object: Object::Cube,
        data: SpawnData {
            transform: Transform::from_xyz(1.0, 2.0, 3.0),
            name: "Cube with struct".to_string(),
        },
    });
}

fn spawn_something_with_tuple(
    mut spawn_events: EventWriter<SpawnEvent<Object, (Transform, String)>>,
) {
    spawn_events.send(SpawnEvent {
        object: Object::Cube,
        data: (
            Transform::from_xyz(4.0, 5.0, 6.0),
            "Cube with tuple".to_string(),
        ),
    });
}

fn spawn_with_struct(data: SpawnData, world: &mut World) {
    info!("Spawning {} at {}", data.name, data.transform.translation);
    world.spawn((Name::new("Cube"), data.transform));
}

fn spawn_with_tuple((transform, name): (Transform, String), world: &mut World) {
    info!("Spawning {} at {}", name, transform.translation);
    world.spawn((Name::new("Cube"), transform));
}
