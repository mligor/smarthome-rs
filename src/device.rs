use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{
    event::{Event, Sender},
    types::RHomeObject,
};

pub trait DeviceInterface: RHomeObject + Send {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn start(&mut self, _tx: Sender) -> bool {
        return true;
    }
    fn stop(&mut self) {}
}

type DeviceValue = Arc<Mutex<Box<dyn DeviceInterface>>>;

#[derive(Clone)]
pub struct Device {
    value: DeviceValue,
}

impl Device {
    pub fn new(data: Box<dyn DeviceInterface>) -> Self {
        Self {
            value: Arc::new(Mutex::new(data)),
        }
    }

    pub fn start(&mut self, tx: Sender) {
        let ev = Event::new(format!("{} started", self.get_name()), self.id());

        let mut rx = tx.subscribe();
        //println!("Starting device {} message loop", self.name());
        _ = tx.send(ev);
        let dev = self.clone();
        let tx2 = tx.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(ev) => dev.handle_event(ev, &tx),
                    Err(err) => println!("error receiving event in device: {}", err),
                }
            }
        });
        self.value.lock().unwrap().start(tx2);
    }

    fn handle_event(&self, ev: Event, _tx: &Sender) {
        if self.id() == ev.source {
            return; // ignore own events
        }
        let name = self.value.lock().unwrap().get_name();

        println!("{}: {}", name, ev.name);
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.value.lock().unwrap().set_name(name);
    }

    pub(crate) fn get_name(&self) -> String {
        self.value.lock().unwrap().get_name()
    }
}

impl RHomeObject for Device {
    fn id(&self) -> Uuid {
        let d = self.value.lock().unwrap();
        d.as_ref().id()
    }
}
