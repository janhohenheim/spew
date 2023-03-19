use bevy::core::FrameCount;
use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Object {
    Cube,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Object, Transform>::default())
        .add_spawner((Object::Cube, spawn_cube))
        .add_systems((spawn_without_delay, spawn_with_delay).on_startup())
        .run();
}

/// This cube will spawn 1 tick after the event is sent
fn spawn_without_delay(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    spawn_events.send(SpawnEvent {
        object: Object::Cube,
        data: Transform::from_xyz(1.0, 2.0, 3.0),
    });
}

/// This cube will spawn 1 tick later than usual, so in total 2 ticks after the event is sent
fn spawn_with_delay(mut spawn_events: EventWriter<DelayedSpawnEvent<Object, Transform>>) {
    spawn_events.send(
        SpawnEvent {
            object: Object::Cube,
            data: Transform::from_xyz(4.0, 5.0, 6.0),
        }
        .with_delay(1),
    );
}

fn spawn_cube(transform: Transform, world: &mut World) {
    let frame_count = world.get_resource::<FrameCount>().unwrap();
    info!(
        "Spawning cube at {} on frame {}",
        transform.translation, frame_count.0
    );
    world.spawn((Name::new("Cube"), transform));
}
