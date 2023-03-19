use bevy::prelude::*;

pub struct SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    pub object: T,
    pub data: D,
}

pub struct DelayedSpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    pub spawn_event: SpawnEvent<T, D>,
    pub delay: usize,
}

impl<T, D> SpawnEvent<T, D>
where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    pub fn with_delay(self, delay: usize) -> DelayedSpawnEvent<T, D> {
        DelayedSpawnEvent {
            spawn_event: self,
            delay,
        }
    }
}

pub(crate) fn delay_spawn_events<T, D>(
    mut delayed_spawn_events: ResMut<Events<DelayedSpawnEvent<T, D>>>,
    mut spawn_event_writer: EventWriter<SpawnEvent<T, D>>,
) where
    T: Eq + Send + Sync + 'static,
    D: Send + Sync + 'static,
{
    let mut advanced_events = Vec::new();
    for event in delayed_spawn_events.drain() {
        if event.delay == 0 {
            spawn_event_writer.send(event.spawn_event);
        } else {
            advanced_events.push(DelayedSpawnEvent {
                spawn_event: event.spawn_event,
                delay: event.delay - 1,
            });
        }
    }
    for event in advanced_events {
        delayed_spawn_events.send(event);
    }
}
