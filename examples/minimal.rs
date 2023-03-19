use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Object {
    Cube,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Object, Transform>::default())
        .add_spawner((Object::Cube, spawn_cube))
        .add_system(spawn_something.on_startup())
        .run();
}

fn spawn_something(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    spawn_events.send(SpawnEvent::new(
        Object::Cube,
        Transform::from_xyz(1.0, 2.0, 3.0),
    ));
}

fn spawn_cube(world: &mut World, transform: Transform) {
    info!("Spawning cube at {}", transform.translation);
    world.spawn((Name::new("Cube"), transform));
}
