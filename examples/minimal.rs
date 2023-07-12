use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Object {
    Cube,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpewPlugin::<Object>::default())
        .add_spawner((Object::Cube, spawn_cube))
        .add_systems(Startup, spawn_something)
        .run();
}

fn spawn_something(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
    spawn_events.send(SpawnEvent::new(Object::Cube));
}

fn spawn_cube(mut commands: Commands) {
    info!("Spawning cube");
    commands.spawn(Name::new("Cube"));
}
