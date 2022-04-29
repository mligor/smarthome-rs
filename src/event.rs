use derive_more::Display;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
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

pub trait EventHandler {
    fn handle_event(&mut self, ev: Event);
    fn create_receiver(&mut self) -> Receiver;
}

pub async fn run_event_loop(handler: Arc<Mutex<Box<impl EventHandler>>>) {
    let mut rx: Receiver;
    {
        let mut h = handler.lock().unwrap();
        rx = h.create_receiver();
    }

    loop {
        match rx.recv().await {
            Ok(ev) => {
                let mut h = handler.lock().unwrap();
                h.handle_event(ev);
            }
            Err(err) => println!("error receiving event in manager: {}", err),
        }
    }
}
