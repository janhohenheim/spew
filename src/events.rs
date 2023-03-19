use bevy::prelude::*;

/// An event that will spawn an object in the world.
/// This is the most common way to interact with the plugin.
/// `T` is the type of the object to spawn, and `D` is the type of the user-provided data.
/// Any combination of `T` and `D` used in a `SpawnEvent` must have been registered with an own [`SpewPlugin`] beforehand.
///
/// # Example
/// ```rust
/// use spew::prelude::*;
/// use bevy::prelude::*;
///
/// #[derive(Debug, Eq, PartialEq)]
/// enum Object {
///    Cube
/// }
///
/// fn spawn_something(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
///    spawn_events.send(SpawnEvent {
///       object: Object::Cube,
///       data: Transform::from_xyz(1.0, 2.0, 3.0),
///   });
/// }
pub struct SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    /// The object to spawn.
    pub object: T,
    /// The user-provided data to pass to the spawner.
    pub data: D,
}

/// An event that will spawn an object in the world after a delay.
/// Although this event can be built manually, the preferred way is to use the convenience methods on [`SpawnEvent`], namely [`SpawnEvent::delay_frames`] and [`SpawnEvent::delay_seconds`].
pub struct DelayedSpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    /// The event that is being delayed.
    pub spawn_event: SpawnEvent<T, D>,
    /// The delay to apply.
    pub delay: Delay,
}

impl<T, D> SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    /// Delay the spawning of the object by a number of frames.
    /// Note that objects are spawned a frame after they are spawned per default, so a delay of 1 means that the object will be spawned 2 frames after the event is sent.
    ///
    /// Setting a frame delay of 0 is the same as spawning it without delay.
    /// To be used in tandem with an [`EventWriter`] of type [`DelayedSpawnEvent`].
    ///
    /// # Example
    /// ```rust
    /// use spew::prelude::*;
    /// use bevy::prelude::*;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// enum Object {
    ///     Cube
    /// }
    ///
    /// fn spawn_with_delay(mut spawn_events: EventWriter<DelayedSpawnEvent<Object, Transform>>) {
    ///     spawn_events.send(
    ///         SpawnEvent {
    ///             object: Object::Cube,
    ///             data: Transform::from_xyz(4.0, 5.0, 6.0),
    ///         }
    ///         .delay_frames(1),
    ///     );
    /// }
    pub fn delay_frames(self, delay: usize) -> DelayedSpawnEvent<T, D> {
        DelayedSpawnEvent {
            spawn_event: self,
            delay: Delay::Frames(delay),
        }
    }

    /// Delay the spawning of the object by a number of seconds.
    /// Since objects are spawned a frame after they are spawned per default, a delay of 0.0 means that the object will be spawned 1 frame after the event is sent.
    ///
    /// To be used in tandem with an [`EventWriter`] of type [`DelayedSpawnEvent`].
    ///
    /// # Example
    /// ```rust
    /// use spew::prelude::*;
    /// use bevy::prelude::*;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// enum Object {
    ///     Cube
    /// }
    ///
    /// fn spawn_with_delay(mut spawn_events: EventWriter<DelayedSpawnEvent<Object, Transform>>) {
    ///     spawn_events.send(
    ///         SpawnEvent {
    ///             object: Object::Cube,
    ///             data: Transform::from_xyz(4.0, 5.0, 6.0),
    ///         }
    ///         .delay_seconds(1.0),
    ///     );
    /// }
    pub fn delay_seconds(self, delay: f32) -> DelayedSpawnEvent<T, D> {
        DelayedSpawnEvent {
            spawn_event: self,
            delay: Delay::Seconds(delay),
        }
    }
}

/// A delay for spawning an object.
pub enum Delay {
    /// Wait for a number of frames longer than usual.
    Frames(usize),
    /// Wait for a number of seconds before spawning the object.
    Seconds(f32),
}

pub(crate) fn delay_spawn_events<T, D>(
    time: Res<Time>,
    mut delayed_spawn_events: ResMut<Events<DelayedSpawnEvent<T, D>>>,
    mut spawn_event_writer: EventWriter<SpawnEvent<T, D>>,
) where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    let mut advanced_events = Vec::new();
    for event in delayed_spawn_events.drain() {
        match event.delay {
            Delay::Frames(delay) => {
                if delay == 0 {
                    spawn_event_writer.send(event.spawn_event);
                } else {
                    advanced_events.push(DelayedSpawnEvent {
                        spawn_event: event.spawn_event,
                        delay: Delay::Frames(delay - 1),
                    });
                }
            }
            Delay::Seconds(delay) => {
                if delay <= 1e-5 {
                    spawn_event_writer.send(event.spawn_event);
                } else {
                    advanced_events.push(DelayedSpawnEvent {
                        spawn_event: event.spawn_event,
                        delay: Delay::Seconds(delay - time.delta_seconds()),
                    });
                }
            }
        }
    }
    for event in advanced_events {
        delayed_spawn_events.send(event);
    }
}
