use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct SpawnEvent<T: Eq + Clone + Send + Sync + 'static> {
    pub object: T,
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub struct DelayedSpawnEvent<T: Eq + Clone + Send + Sync + 'static> {
    pub spawn_event: SpawnEvent<T>,
    pub delay: usize,
}

impl<T: Eq + Clone + Send + Sync + 'static> SpawnEvent<T> {
    pub fn with_delay(self, delay: usize) -> DelayedSpawnEvent<T> {
        DelayedSpawnEvent {
            spawn_event: self,
            delay,
        }
    }
}

pub(crate) fn delay_spawn_events<T: Eq + Clone + Send + Sync + 'static>(
    mut delayed_spawn_events: ParamSet<(
        EventReader<DelayedSpawnEvent<T>>,
        EventWriter<DelayedSpawnEvent<T>>,
    )>,
    mut spawn_event_writer: EventWriter<SpawnEvent<T>>,
) {
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
