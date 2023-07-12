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
        .add_plugins(SpewPlugin::<Object, String>::default())
        .add_spawners((
            (Object::Cube, spawn_cube_with_transform),
            (Object::Cube, spawn_cube_with_name),
        ))
        .add_systems(Startup, spawn_something_with_transform)
        .run();
}

fn spawn_something_with_transform(
    mut spawn_with_transform_events: EventWriter<SpawnEvent<Object, Transform>>,
    mut spawn_with_name_events: EventWriter<SpawnEvent<Object, String>>,
) {
    spawn_with_transform_events.send(SpawnEvent::with_data(
        Object::Cube,
        Transform::from_xyz(1.0, 2.0, 3.0),
    ));

    spawn_with_name_events.send(SpawnEvent::with_data(
        Object::Cube,
        "a very very friendly cube".to_owned(),
    ));
}

fn spawn_cube_with_transform(In(transform): In<Transform>, mut commands: Commands) {
    info!("Spawning cube at {}", transform.translation);
    commands.spawn((Name::new("Cube"), transform));
}

fn spawn_cube_with_name(In(name): In<String>, mut commands: Commands) {
    info!("Spawning {}", name);
    commands.spawn(Name::new(name));
}
