use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub source: Uuid,
    pub source_name: String,
}

pub type Sender = broadcast::Sender<Event>;
pub type Receiver = broadcast::Receiver<Event>;

pub fn channel() -> (Sender, Receiver) {
    broadcast::channel::<Event>(1024)
}

impl Event {
    pub(crate) fn new(name: String, source: Uuid, source_name: String) -> Event {
        Event {
            name,
            source,
            source_name,
        }
    }
}
