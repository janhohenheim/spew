use bevy::prelude::*;
use spew::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum Creature {
    Human,
    Cow,
    Zombie,
}

#[derive(Debug, Eq, PartialEq)]
enum Furniture {
    Chair,
    Table,
    Bed,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpewPlugin::<Creature>::default())
        .add_plugin(SpewPlugin::<Furniture>::default())
        // This can also be done with two separate calls to add_spawners, if you prefer
        .add_spawners((
            (Creature::Human, spawn_human),
            (Creature::Cow, spawn_cow),
            (Creature::Zombie, spawn_zombie),
            (Furniture::Chair, spawn_chair),
            (Furniture::Table, spawn_table),
            (Furniture::Bed, spawn_bed),
        ))
        .add_systems((spawn_creatures, spawn_furniture).on_startup())
        .run();
}

fn spawn_creatures(mut spawn_events: EventWriter<SpawnEvent<Creature>>) {
    spawn_events.send(SpawnEvent::new(Creature::Human));
    spawn_events.send(SpawnEvent::new(Creature::Cow));
    spawn_events.send(SpawnEvent::new(Creature::Zombie));
}

fn spawn_furniture(mut spawn_events: EventWriter<SpawnEvent<Furniture>>) {
    spawn_events.send(SpawnEvent::new(Furniture::Chair));
    spawn_events.send(SpawnEvent::new(Furniture::Table));
    spawn_events.send(SpawnEvent::new(Furniture::Bed));
}

fn spawn_human(mut commands: Commands) {
    info!("Spawning human");
    commands.spawn(Name::new("Human"));
}

fn spawn_cow(mut commands: Commands) {
    info!("Spawning cow");
    commands.spawn(Name::new("Cow"));
}

fn spawn_zombie(mut commands: Commands) {
    info!("Spawning zombie");
    commands.spawn(Name::new("Zombie"));
}

fn spawn_chair(mut commands: Commands) {
    info!("Spawning chair");
    commands.spawn(Name::new("Chair"));
}

fn spawn_table(mut commands: Commands) {
    info!("Spawning table");
    commands.spawn(Name::new("Table"));
}

fn spawn_bed(mut commands: Commands) {
    info!("Spawning bed");
    commands.spawn(Name::new("Bed"));
}
