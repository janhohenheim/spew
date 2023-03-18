use bevy::prelude::*;

pub struct SpawnEvent<T: Eq + Send + Sync + 'static> {
    pub object: T,
    pub transform: Transform,
}
