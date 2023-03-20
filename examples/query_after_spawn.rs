use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum Object {
    Player,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Object>::default())
        .add_spawner((Object::Player, spawn_player))
        .add_systems((setup.on_startup(), query_player.after(SpewSystemSet)))
        .run();
}

#[derive(Component)]
struct Player {
    name: String,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player {
        name: "Franz Ferdinand".to_string(),
    });
}

fn setup(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
    spawn_events.send(SpawnEvent::new(Object::Player));
}

fn query_player(player: Query<&Player>) {
    let player = player.single();
    info!("Found a player named {}", player.name);
}
