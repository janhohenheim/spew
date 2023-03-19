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
///    spawn_events.send(SpawnEvent::new(
///       Object::Cube,
///       Transform::from_xyz(1.0, 2.0, 3.0),
///   ));
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
    /// The delay to apply.
    pub delay: Delay,
}

impl Default for Delay {
    fn default() -> Self {
        Self::Frames(0)
    }
}

impl<T, D> Default for SpawnEvent<T, D>
where
    T: Eq + Send + Sync + Default + 'static,
    D: Send + Sync + Default + 'static,
{
    fn default() -> Self {
        Self {
            object: default(),
            data: default(),
            delay: default(),
        }
    }
}

/// A trait that allows creating a [`SpawnEvent`] without user-provided data.
pub trait NewSpawnEventWithoutData<T> {
    /// Create a new spawn event
    #[allow(clippy::new_ret_no_self)]
    fn new(object: T) -> SpawnEvent<T, ()>
    where
        T: Eq + Send + Sync + 'static;
}

impl<T> NewSpawnEventWithoutData<T> for SpawnEvent<T, ()>
where
    T: Eq + Send + Sync + 'static,
{
    fn new(object: T) -> SpawnEvent<T, ()> {
        SpawnEvent::new(object, ())
    }
}

impl<T, D> SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    /// Create a new spawn event with user data and no delay.
    pub fn new(object: T, data: D) -> Self {
        Self {
            object,
            data,
            delay: default(),
        }
    }

    /// Delay the spawning of the object by a number of frames.
    /// Note that objects are spawned a frame after they are spawned per default, so a delay of 1 means that the object will be spawned 2 frames after the event is sent.
    ///
    /// Setting a frame delay of 0 is the same as spawning it without delay, which is the default behavior.
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
    /// fn spawn_with_delay(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    ///     spawn_events.send(
    ///         SpawnEvent::new(
    ///             Object::Cube,
    ///             Transform::from_xyz(4.0, 5.0, 6.0),
    ///         )
    ///         .delay_frames(1),
    ///     );
    /// }
    pub fn delay_frames(mut self, delay: usize) -> SpawnEvent<T, D> {
        self.delay = Delay::Frames(delay);
        self
    }

    /// Delay the spawning of the object by a number of seconds.
    /// Since objects are spawned a frame after they are spawned per default, a delay of 0.0 means that the object will be spawned 1 frame after the event is sent.
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
    /// fn spawn_with_delay(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
    ///     spawn_events.send(
    ///         SpawnEvent::new(
    ///             Object::Cube,
    ///             Transform::from_xyz(4.0, 5.0, 6.0),
    ///         )
    ///         .delay_seconds(1.0),
    ///     );
    /// }
    pub fn delay_seconds(mut self, delay: f32) -> Self {
        self.delay = Delay::Seconds(delay);
        self
    }
}

/// A delay for spawning an object. The default is no delay.
pub enum Delay {
    /// Wait for a number of frames longer than usual.
    Frames(usize),
    /// Wait for a number of seconds before spawning the object.
    Seconds(f32),
}

pub(crate) fn delay_spawn_events<T, D>(
    time: Res<Time>,
    mut delayed_spawn_events: ResMut<Events<SpawnEvent<T, D>>>,
    mut spawn_event_writer: EventWriter<ReadySpawnEvent<T, D>>,
) where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    let mut advanced_events = Vec::new();
    for event in delayed_spawn_events.drain() {
        match event.delay {
            Delay::Frames(delay) => {
                if delay == 0 {
                    spawn_event_writer.send(event.into());
                } else {
                    advanced_events.push(SpawnEvent {
                        delay: Delay::Frames(delay - 1),
                        ..event
                    });
                }
            }
            Delay::Seconds(delay) => {
                if delay <= 1e-5 {
                    spawn_event_writer.send(event.into());
                } else {
                    advanced_events.push(SpawnEvent {
                        delay: Delay::Seconds(delay - time.delta_seconds()),
                        ..event
                    });
                }
            }
        }
    }
    for event in advanced_events {
        delayed_spawn_events.send(event);
    }
}

pub(crate) struct ReadySpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    pub(crate) object: T,
    pub(crate) data: D,
}

impl<T, D> From<SpawnEvent<T, D>> for ReadySpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    fn from(event: SpawnEvent<T, D>) -> Self {
        Self {
            object: event.object,
            data: event.data,
        }
    }
}
