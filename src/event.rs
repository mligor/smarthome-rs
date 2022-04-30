use derive_more::Display;
use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::Ptr;

pub type EventData = HashMap<String, String>;

#[derive(Clone, Display)]
#[display(fmt = "src={}, event={}, data={:?}", source, name, data)]
pub(crate) struct Event {
    pub name: String,
    pub data: EventData,
    pub source: String,
}

pub(crate) type Sender = broadcast::Sender<Event>;
pub(crate) type Receiver = broadcast::Receiver<Event>;

pub(crate) fn channel() -> (Sender, Receiver) {
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

pub(crate) trait EventHandler {
    fn handle_event(&mut self, _ev: Event) {}
}

pub(crate) async fn run_event_loop(mut rx: Receiver, handler: Ptr<impl EventHandler + ?Sized>) {
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

pub(crate) trait EventSender {
    fn get_receiver(&self) -> Receiver;
}
