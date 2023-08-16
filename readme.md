# Spew

[![crates.io](https://img.shields.io/crates/v/spew)](https://crates.io/crates/spew)
[![docs.rs](https://docs.rs/spew/badge.svg)](https://docs.rs/spew)

A simple helper for spawning objects in Bevy.

## Usage

First, create an `enum` that holds objects you might want to spawn:

```rust
#[derive(Debug, Eq, PartialEq)]
enum Objects {
    Player,
    Monster,
    Coin,
}
```

Think about which data you want to pass to the spawning function. In this example, we will specify a `Transform` for the new object.
Next, add the plugin to your app, noting the two types we just mentioned:

```rust,ignore
use spew::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
    // ...
        .add_plugins(SpewPlugin::<Objects, Transform>::default()) // <--- Add the plugin
    // ...
        .run();
}
```

Now, we are ready to register our spawn functions. Each variant of the `enum` will be associated with its own spawn function that takes in a `&mut World` and the user provided data:
```rust,ignore
use spew::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
    // ...
        .add_spawners( // <--- Register the spawn functions
            (Objects::Player, spawn_player),
            (Objects::Monster, spawn_monster),
            (Objects::Coin, spawn_coin),
        )
    // ...
        .run();
}

fn spawn_player(In(transform): In<Transform>, mut commands: Commands) { {
    commands.spawn((
        Name::new("Spiffy the Adventurer"),
        TransformBundle::from_transform(transform),
    ));
}

fn spawn_monster(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        Name::new("Grumblor the Grumpy"),
        TransformBundle::from_transform(transform),
    ));
}

fn spawn_coin(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        Name::new("$1000"),
        TransformBundle::from_transform(transform),
    ));
}
```

Finally, we can set our spawn functions to work by sending a `SpawnEvent`:
```rust,ignore
use spew::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
    // ...
        .add_systems(Startup, setup_map)
    // ...
        .run();
}

fn setup_map(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    spawn_events.send(SpawnEvent::with_data(
        Objects::Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    spawn_events.send(SpawnEvent::with_data(
        Objects::Monster,
        Transform::from_xyz(5.0, 0.0, 0.0),
    ));
    spawn_events.send(SpawnEvent::with_data(
        Objects::Coin,
        Transform::from_xyz(10.0, 0.0, 0.0),
    ));
}
```

You can read through the [docs](https://docs.rs/spew) or peruse the [examples](https://github.com/janhohenheim/spew/examples) for more use cases.
Other cool stuff you can do is delay the spawning by a certain amount of frames or time or organize your spawn lists into multiple enums.

## Compatibility
| bevy | spew  |
|------|-------|
| 0.10 | 0.2.2 |
| 0.11 | 0.3.0 |


## Motivation

Bevy's `Commands` API allows you to spawn new entities with arbitrary components:
```rust
use bevy::prelude::*;

fn spawn_player(commands: &mut Commands) {
    commands.spawn((
        Name::new("Adventurer"),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
}
```
This works great! We can spawn more complex objects by just adding more components like assets:
```rust
use std::f32::consts::TAU;
use bevy::prelude::*;

fn spawn_bullet(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Bullet"),
        SceneBundle {
            scene: asset_server.load("models/bullet.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(5.0, 4.0, 12.0),
                scale: Vec3::splat(0.012),
                rotation: Quat::from_rotation_y(TAU / 2.),
            },
            ..default()
        },
    ));
}
```
but, in a real project, we would not spawn a bullet like that. The bullet would be spawned by a weapon at a certain translation.
We might thus encapsulate the bullet spawning like this:
```rust,ignore
use bevy::prelude::*;
fn handle_input(...) {
    // ...
    if should_fire_bullet {
        let position = player_transform.translation;
        spawn_bullet(&mut commands, &asset_server, position);
    }
}

fn spawn_bullet(commands: &mut Commands, asset_server: &AssetServer, position: Vec3) {
    commands.spawn((
        Name::new("Bullet"),
        SceneBundle {
            scene: asset_server.load("models/bullet.gltf#Scene0"),
            transform: Transform {
                translation: position,
                scale: Vec3::splat(0.012),
                rotation: Quat::from_rotation_y(TAU / 2.),
            },
            ..default()
        },
    ));
}
```

As you can see, this works but is quite ugly. `handle_input` has to pass around an asset server we might otherwise not even need in the system,
and `spawn_bullet` has a jumble of seemingly unrelated parameters that will grow and grow over time. Growing parameter lists are not a problem
when writing a system, but notice how here `spawn_bullet` is no longer a system but a helper function. Thus, its call will get longer and uglier over time,
with all its parameters leaking into `handle_input`.

The solution to this is to move the spawning of the bullet into an own system that is accessed indirectly by `handle_input` via events, which is just what this crate helps you with! :) 
