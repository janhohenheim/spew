use crate::events::{delay_spawn_events, ReadySpawnEvent, SpawnEvent};
use crate::spawner::{Spawner, Spawners};
use bevy::prelude::*;

#[allow(clippy::needless_doctest_main)]
/// A plugin that enables spawning objects of type `T` while providing data of type `D`.
/// Using multiple combinations of `T` and `D` requires adding multiple instances of this plugin to an [`App`].
/// If your spawn systems don't require any data, simply pass `()` as the `D` type.
///
/// # Example
/// ```rust,ignore
/// use spew::prelude::*;
/// use bevy::prelude::*;
///
/// #[derive(Debug, Eq, PartialEq)]
/// enum Object {
///    Cube
/// }
///
/// fn main() {
///    App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugin(SpewPlugin::<Object, Transform>::default())
///      .run();
/// }
pub struct SpewPlugin<T, D = ()>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    _spawner_enum_type: std::marker::PhantomData<T>,
    _data_type: std::marker::PhantomData<D>,
}

impl<T, D> Default for SpewPlugin<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            _spawner_enum_type: std::marker::PhantomData,
            _data_type: std::marker::PhantomData,
        }
    }
}

impl<T, D> Plugin for SpewPlugin<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent<T, D>>()
            .add_event::<ReadySpawnEvent<T, D>>()
            .add_system(delay_spawn_events::<T, D>.in_set(DelayerSystemSet));
    }

    fn is_unique(&self) -> bool {
        false
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct DelayerSystemSet;

/// A trait that allows adding spawners to an [`App`].
/// Spawners are tuples of an object and a spawning function, e.g. `(Object::Cube, spawn_cube)`. A spawning function has the signature `fn(&mut Commands, D)`, where D is any user provided data.
///
/// The spawner's combination of object enum and user data must have been registered with an own [`SpewPlugin`] beforehand.
pub trait SpewApp {
    /// Add a single spawner to the app.
    ///
    /// # Example
    /// ```rust,ignore
    /// use spew::prelude::*;
    /// use bevy::prelude::*;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// enum Object {
    ///   Cube
    /// }
    ///
    /// fn main() {
    ///     App::new()
    ///         .add_plugins(DefaultPlugins)
    ///         .add_plugin(SpewPlugin::<Object, Transform>::default())
    ///         .add_spawner((Object::Cube, spawn_cube))
    ///         .run();
    /// }
    ///
    /// fn spawn_cube(world: &mut World, transform: Transform) {
    ///    info!("Spawning cube at {}", transform.translation);
    ///    world.spawn((Name::new("Cube"), transform));
    /// }
    /// ```
    fn add_spawner<T, D>(&mut self, spawner: T) -> &mut App
    where
        T: Spawner<D>;

    /// Add multiple spawners to the app by providing them in a tuple.
    ///
    /// # Example
    /// ```rust,ignore
    /// use spew::prelude::*;
    /// use bevy::prelude::*;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// enum Object {
    ///   Cube,
    ///   Triangle,
    ///   Sphere,
    /// }
    ///
    /// fn main() {
    ///     App::new()
    ///         .add_plugins(DefaultPlugins)
    ///         .add_plugin(SpewPlugin::<Object, Transform>::default())
    ///         .add_spawners((
    ///             (Object::Cube, spawn_cube),
    ///             (Object::Triangle, spawn_triangle),
    ///             (Object::Sphere, spawn_sphere),
    ///         ))
    ///         .run();
    /// }
    ///
    /// fn spawn_cube(world: &mut World, transform: Transform) {
    ///    info!("Spawning cube at {}", transform.translation);
    ///    world.spawn((Name::new("Cube"), transform));
    /// }
    ///
    /// fn spawn_triangle(world: &mut World, transform: Transform) {
    ///    info!("Spawning triangle at {}", transform.translation);
    ///    world.spawn((Name::new("Cube"), transform));
    /// }
    ///
    /// fn spawn_sphere(world: &mut World, transform: Transform) {
    ///    info!("Spawning sphere at {}", transform.translation);
    ///    world.spawn((Name::new("Cube"), transform));
    /// }
    /// ```
    fn add_spawners<T, D>(&mut self, spawners: T) -> &mut App
    where
        T: Spawners<D>;
}

impl SpewApp for App {
    fn add_spawner<T, D>(&mut self, spawner: T) -> &mut App
    where
        T: Spawner<D>,
    {
        spawner.add_to_app(self);
        self
    }
    fn add_spawners<T, D>(&mut self, spawners: T) -> &mut App
    where
        T: Spawners<D>,
    {
        spawners.add_to_app(self);
        self
    }
}
