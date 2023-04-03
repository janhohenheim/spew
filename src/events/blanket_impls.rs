use crate::events::SpawnEvent;
use std::fmt::{Debug, Formatter};

impl<T, D> Clone for SpawnEvent<T, D>
where
    T: Eq + Send + Sync + Clone + 'static,
    D: Send + Sync + Clone + 'static,
{
    fn clone(&self) -> Self {
        Self {
            object: self.object.clone(),
            data: self.data.clone(),
            delay: self.delay.clone(),
        }
    }
}

impl<T, D> Debug for SpawnEvent<T, D>
where
    T: Eq + Send + Sync + Debug + 'static,
    D: Send + Sync + Debug + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadySpawnEvent")
            .field("object", &self.object)
            .field("data", &self.data)
            .field("delay", &self.delay)
            .finish()
    }
}

impl<T, D> PartialEq for SpawnEvent<T, D>
where
    T: Eq + Send + Sync + PartialEq + 'static,
    D: Send + Sync + PartialEq + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.data == other.data && self.delay == other.delay
    }
}

impl<T, D> Default for SpawnEvent<T, D>
where
    T: Eq + Send + Sync + Default + 'static,
    D: Send + Sync + Default + 'static,
{
    fn default() -> Self {
        Self {
            object: Default::default(),
            data: Default::default(),
            delay: Default::default(),
        }
    }
}
