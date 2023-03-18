use bevy::prelude::*;
use spew::prelude::*;

#[derive(Eq, PartialEq)]
enum Object {
    Cube,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Object>::default())
        .add_spawners(((Object::Cube, spawn_cube),))
        .add_startup_system(setup)
        .run();
}

fn setup(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
    spawn_events.send(SpawnEvent {
        object: Object::Cube,
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
    });
}

fn spawn_cube(transform: Transform, world: &mut World) {
    info!("Spawning cube at {:?}", transform.translation);
    world.spawn((Name::new("Cube"), transform));
}
