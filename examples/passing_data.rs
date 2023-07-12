use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Object {
    Cube,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpewPlugin::<Object, Transform>::default())
        .add_spawners(((Object::Cube, spawn_cube_with_transform),))
        .add_systems(Startup, spawn_something_with_transform)
        .run();
}

fn spawn_something_with_transform(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    spawn_events.send(SpawnEvent::with_data(
        Object::Cube,
        Transform::from_xyz(1.0, 2.0, 3.0),
    ));
}

fn spawn_cube_with_transform(In(transform): In<Transform>, mut commands: Commands) {
    info!("Spawning cube at {}", transform.translation);
    commands.spawn((Name::new("Cube"), transform));
}
