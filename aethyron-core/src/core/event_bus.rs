use crate::core::events::Event;

pub struct EventBus;

impl EventBus {
    pub fn new() -> Self {
        Self
    }

    pub fn publish(&self, event: Event) {
        println!(
            "[{}] [{}] {}",
            event.timestamp,
            event.source,
            event.message
        );
    }
}