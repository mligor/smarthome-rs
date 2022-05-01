use derive_more::Display;
use std::{collections::HashMap, fmt::Debug};
use tokio::sync::broadcast;

use crate::Ptr;

pub type EventData = HashMap<&'static str, EventDataValue>;

#[derive(Clone)]
pub enum EventDataValue {
    String(String),
    Bool(bool),
    Number(f32),
}

impl Debug for EventDataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(d) => write!(f, "\"{}\"", d),
            Self::Bool(d) => write!(f, "{}", d),
            Self::Number(d) => write!(f, "{}", d),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Display, PartialEq)]
pub enum EventTarget {
    Everyone,
    EveryoneIncludeSender,
    SenderOnly,
    ManagerOnly,
}

#[derive(Clone, Display)]
#[display(
    fmt = "src={}, event={}, trg={}, data={:?}",
    source,
    name,
    target,
    data
)]
pub(crate) struct Event {
    pub name: String,
    pub data: EventData,
    pub source: String,
    pub target: EventTarget,
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
            target: EventTarget::Everyone,
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
