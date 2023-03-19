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
        .add_plugin(SpewPlugin::<Creature, Transform>::default())
        .add_plugin(SpewPlugin::<Furniture, Transform>::default())
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

fn spawn_creatures(mut spawn_events: EventWriter<SpawnEvent<Creature, Transform>>) {
    spawn_events.send(SpawnEvent {
        object: Creature::Human,
        data: Transform::from_xyz(1.0, 2.0, 3.0),
    });
    spawn_events.send(SpawnEvent {
        object: Creature::Cow,
        data: Transform::from_xyz(4.0, 2.0, 1.0),
    });
    spawn_events.send(SpawnEvent {
        object: Creature::Zombie,
        data: Transform::from_xyz(6.0, 2.0, 5.0),
    });
}

fn spawn_furniture(mut spawn_events: EventWriter<SpawnEvent<Furniture, Transform>>) {
    spawn_events.send(SpawnEvent {
        object: Furniture::Chair,
        data: Transform::from_xyz(1.0, 2.0, 3.0),
    });
    spawn_events.send(SpawnEvent {
        object: Furniture::Table,
        data: Transform::from_xyz(4.0, 2.0, 1.0),
    });
    spawn_events.send(SpawnEvent {
        object: Furniture::Bed,
        data: Transform::from_xyz(6.0, 2.0, 5.0),
    });
}

fn spawn_human(world: &mut World, transform: Transform) {
    info!("Spawning human at {}", transform.translation);
    world.spawn((Name::new("Human"), transform));
}

fn spawn_cow(world: &mut World, transform: Transform) {
    info!("Spawning cow at {}", transform.translation);
    world.spawn((Name::new("Cow"), transform));
}

fn spawn_zombie(world: &mut World, transform: Transform) {
    info!("Spawning zombie at {}", transform.translation);
    world.spawn((Name::new("Zombie"), transform));
}

fn spawn_chair(world: &mut World, transform: Transform) {
    info!("Spawning chair at {}", transform.translation);
    world.spawn((Name::new("Chair"), transform));
}

fn spawn_table(world: &mut World, transform: Transform) {
    info!("Spawning table at {}", transform.translation);
    world.spawn((Name::new("Table"), transform));
}

fn spawn_bed(world: &mut World, transform: Transform) {
    info!("Spawning bed at {}", transform.translation);
    world.spawn((Name::new("Bed"), transform));
}
