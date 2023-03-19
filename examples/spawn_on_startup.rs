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
        .add_system(setup.on_startup())
        .add_system(query_spawned_player)
        .run();
}

#[derive(Component)]
struct Player {
    pub name: String,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player {
        name: "Carl".to_string(),
    });
}

fn setup(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
    spawn_events.send(SpawnEvent::new(Object::Player));
}

fn query_spawned_player(player: Query<&Player>) {
    let player = player.single();
    info!("Found a player called {}", player.name);
}
