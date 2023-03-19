# Spew

A simple helper for spawning objects in Bevy.


## Usage

First, create an `enum` that holds objects you might want to spawn:

```rust
enum Objects {
    Player,
    Monster,
    Coin,
}
```

then, add the plugin to your app:

```rust

```


## Motivation

Bevy's `Commands` API allows you to spawn new entities with arbitrary components:
```rust
fn spawn_player(commands: &mut Commands) {
    commands.spawn((
        Name::new("Adventurer"),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
}
```
This works great! We can spawn more complex objects by just adding more components like assets:
```rust
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
```rust
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