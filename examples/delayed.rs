use bevy::core::FrameCount;
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
        .add_system(spawn_various_delays.on_startup())
        .run();
}

fn spawn_various_delays(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    // This cube will spawn 1 tick after the event is sent
    spawn_events.send(SpawnEvent::new(
        Object::Cube,
        Transform::from_xyz(1.0, 2.0, 3.0),
    ));

    // This cube will spawn 1 tick later than usual, so in total 2 ticks after the event is sent
    spawn_events
        .send(SpawnEvent::new(Object::Cube, Transform::from_xyz(4.0, 5.0, 6.0)).delay_frames(1));

    // This cube will spawn after 0.5
    spawn_events.send(
        SpawnEvent::new(Object::Cube, Transform::from_xyz(10.0, 11.0, 12.0)).delay_seconds(0.5),
    );
}

fn spawn_cube(world: &mut World, transform: Transform) {
    let frame_count = world.get_resource::<FrameCount>().unwrap();
    let time = world.get_resource::<Time>().unwrap();
    info!(
        "Spawning cube at {} on frame {} at time {}",
        transform.translation,
        frame_count.0,
        time.elapsed_seconds()
    );
    world.spawn((Name::new("Cube"), transform));
}
