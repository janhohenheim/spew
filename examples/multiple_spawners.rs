use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq)]
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
    spawn_events.send(SpawnEvent::new(Object::Cube));
    spawn_events.send(SpawnEvent::new(Object::Triangle));
    spawn_events.send(SpawnEvent::new(Object::Sphere));
}

fn spawn_cube(mut commands: Commands) {
    info!("Spawning cube");
    commands.spawn(Name::new("Cube"));
}

fn spawn_triangle(mut commands: Commands) {
    info!("Spawning triangle");
    commands.spawn(Name::new("Triangle"));
}

fn spawn_sphere(mut commands: Commands) {
    info!("Spawning sphere");
    commands.spawn(Name::new("Sphere"));
}
