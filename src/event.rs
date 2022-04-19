use crate::device::Device;
use tokio::sync::broadcast;

pub type Sender = broadcast::Sender<Event>;
pub type Receiver = broadcast::Receiver<Event>;

pub fn channel() -> (Sender, Receiver) {
    broadcast::channel::<Event>(1024)
}

pub trait EventSource: Copy {}

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub source: Option<Device>,
}

impl Event {
    pub fn new(name: String, source: Option<Device>) -> Event {
        Event { name, source }
    }
}

// pub enum Command {
//     Get { key: String },
//     Set { key: String, val: Bytes },
// }
