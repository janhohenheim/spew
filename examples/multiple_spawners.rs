use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Object {
    Cube,
    Triangle,
    Sphere,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Object>::default())
        .add_spawners((
            (Object::Cube, spawn_cube),
            (Object::Triangle, spawn_triangle),
            (Object::Sphere, spawn_sphere),
        ))
        .add_system(spawn_multiple_objects.on_startup())
        .run();
}

fn spawn_multiple_objects(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
    spawn_events.send(SpawnEvent {
        object: Object::Cube,
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
    });
    spawn_events.send(SpawnEvent {
        object: Object::Triangle,
        transform: Transform::from_xyz(4.0, 2.0, 1.0),
    });
    spawn_events.send(SpawnEvent {
        object: Object::Sphere,
        transform: Transform::from_xyz(6.0, 2.0, 5.0),
    });
}

fn spawn_cube(transform: Transform, world: &mut World) {
    info!("Spawning cube at {}", transform.translation);
    world.spawn((Name::new("Cube"), transform));
}

fn spawn_triangle(transform: Transform, world: &mut World) {
    info!("Spawning triangle at {}", transform.translation);
    world.spawn((Name::new("Triangle"), transform));
}

fn spawn_sphere(transform: Transform, world: &mut World) {
    info!("Spawning sphere at {}", transform.translation);
    world.spawn((Name::new("Sphere"), transform));
}
