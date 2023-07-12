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
        .add_plugins(SpewPlugin::<Object>::default())
        .add_spawner((Object::Cube, spawn_cube))
        .add_systems(Startup, spawn_various_delays)
        .run();
}

fn spawn_various_delays(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
    // This cube will spawn in this frame
    spawn_events.send(SpawnEvent::new(Object::Cube));

    // This cube will spawn in the next frame
    spawn_events.send(SpawnEvent::new(Object::Cube).delay_frames(1));

    // This cube will spawn after 0.5
    spawn_events.send(SpawnEvent::new(Object::Cube).delay_seconds(0.5));
}

fn spawn_cube(mut commands: Commands, frame_count: Res<FrameCount>, time: Res<Time>) {
    info!(
        "Spawning cube on frame {} at time {}",
        frame_count.0,
        time.elapsed_seconds()
    );
    commands.spawn(Name::new("Cube"));
}
