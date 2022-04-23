use derive_more::Display;
use std::collections::HashMap;
use tokio::sync::broadcast;

pub type EventData = HashMap<String, String>;

#[derive(Clone, Display)]
#[display(fmt = "src={}, event={}, data={:?}", source, name, data)]
pub struct Event {
    pub name: String,
    pub data: EventData,
    pub source: String,
}

pub type Sender = broadcast::Sender<Event>;
pub type Receiver = broadcast::Receiver<Event>;

pub fn channel() -> (Sender, Receiver) {
    broadcast::channel::<Event>(1024)
}

impl Event {
    pub(crate) fn new(name: String, source: String) -> Event {
        Event {
            name,
            data: EventData::new(),
            source,
        }
    }
}
