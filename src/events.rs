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
/// fn spawn_without_data(mut spawn_events: EventWriter<SpawnEvent<Object>>) {
///    spawn_events.send(SpawnEvent::new(Object::Cube));
/// }
///
/// fn spawn_with_data(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
///    spawn_events.send(SpawnEvent::with_data(
///       Object::Cube,
///       Transform::from_xyz(1.0, 2.0, 3.0),
///   ));
/// }
///
/// fn spawn_with_delay(mut spawn_events: EventWriter<SpawnEvent<Object, Transform>>) {
///    spawn_events.send(SpawnEvent::with_data(
///       Object::Cube,
///       Transform::from_xyz(1.0, 2.0, 3.0),
///   ).delay_frames(10));
/// }
pub struct SpawnEvent<T, D = ()>
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
        Self::new(default())
    }
}

impl<T, D> SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Default + Send + Sync + 'static,
{
    /// Create a new `SpawnEvent` with the given object.
    /// The data will be set to its default value.
    /// Use this if you don't need to pass any data to the spawner or plan on initializing the data later with [`SpawnEvent::data`].
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
    /// let spawn_event: SpawnEvent<Object> = SpawnEvent::new(Object::Cube);
    /// assert_eq!(spawn_event.object, Object::Cube);
    /// assert_eq!(spawn_event.data, ());
    /// ```
    pub fn new(object: T) -> SpawnEvent<T, D> {
        SpawnEvent {
            object,
            data: default(),
            delay: default(),
        }
    }
}

impl<T, D> SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    /// Create a new `SpawnEvent` with the given object and data.
    ///
    /// # Example
    /// ```rust
    /// use spew::prelude::*;
    /// use bevy::prelude::*;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// enum Object {
    ///   Cube
    /// }
    ///
    /// let spawn_event = SpawnEvent::with_data(Object::Cube, Name::new("Dirt Block"));
    /// assert_eq!(spawn_event.object, Object::Cube);
    /// assert_eq!(spawn_event.data, Name::new("Dirt Block"));
    pub fn with_data(object: T, data: D) -> SpawnEvent<T, D> {
        SpawnEvent {
            object,
            data,
            delay: default(),
        }
    }

    /// Delay the spawning of the object by a number of frames.
    /// Setting a frame delay of 0 means spawning in this frame, which is the default behavior.
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
    ///         SpawnEvent::with_data(Object::Cube, Transform::from_xyz(4.0, 5.0, 6.0)).delay_frames(1)
    ///     );
    /// }
    pub fn delay_frames(mut self, delay: usize) -> SpawnEvent<T, D> {
        self.delay = Delay::Frames(delay);
        self
    }

    /// Delay the spawning of the object by a number of seconds.
    /// A delay of 0.0 means that the object will be spawned in this frame.
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
    /// let spawn_event: SpawnEvent<Object> = SpawnEvent::new(Object::Cube).delay_seconds(1.0);
    /// assert_eq!(spawn_event.object, Object::Cube);
    /// assert!(matches!(spawn_event.delay, Delay::Seconds(_)));
    /// ```
    pub fn delay_seconds(mut self, delay: f32) -> Self {
        self.delay = Delay::Seconds(delay);
        self
    }

    /// Change the provided data. This is useful when using [`SpawnEvent::new`], since it initializes the data with the default value.
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
    /// let spawn_event: SpawnEvent<Object, Name> = SpawnEvent::new(Object::Cube).data(Name::new("Dirt Block"));
    /// assert_eq!(spawn_event.object, Object::Cube);
    /// assert_eq!(spawn_event.data, Name::new("Dirt Block"));
    /// ```
    pub fn data(mut self, data: D) -> Self {
        self.data = data;
        self
    }
}

/// A delay for spawning an object. The default is no delay.
pub enum Delay {
    /// Wait for a number of frames before spawning.
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
