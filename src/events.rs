use bevy::prelude::*;

#[derive(Clone)]
pub struct SpawnEvent<T, D>
where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    pub object: T,
    pub data: D,
}

#[derive(Clone)]
pub struct DelayedSpawnEvent<T, D>
where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    pub spawn_event: SpawnEvent<T, D>,
    pub delay: usize,
}

impl<T, D> SpawnEvent<T, D>
where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    pub fn with_delay(self, delay: usize) -> DelayedSpawnEvent<T, D> {
        DelayedSpawnEvent {
            spawn_event: self,
            delay,
        }
    }
}

pub(crate) fn delay_spawn_events<T, D>(
    mut delayed_spawn_events: ParamSet<(
        EventReader<DelayedSpawnEvent<T, D>>,
        EventWriter<DelayedSpawnEvent<T, D>>,
    )>,
    mut spawn_event_writer: EventWriter<SpawnEvent<T, D>>,
) where
    T: Eq + Clone + Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
{
    let mut advanced_events = Vec::new();
    for event in delayed_spawn_events.p0().iter() {
        if event.delay == 0 {
            spawn_event_writer.send(event.spawn_event.clone());
        } else {
            advanced_events.push(DelayedSpawnEvent {
                spawn_event: event.spawn_event.clone(),
                delay: event.delay - 1,
            });
        }
    }
    for event in advanced_events {
        delayed_spawn_events.p1().send(event);
    }
}
