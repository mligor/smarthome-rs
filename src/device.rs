use std::sync::{Arc, Mutex};

use crate::event::{Event, Sender};

pub struct DeviceData {
    pub name: String,
}

pub struct Device {
    value: Arc<Mutex<DeviceData>>,
}

impl Device {
    pub fn new(name: String) -> Self {
        Self {
            value: Arc::new(Mutex::new(DeviceData { name })),
        }
    }

    pub fn start(&mut self, tx: Sender) {
        let ev = Event::new(format!("{} started", self.name()), Some(self.clone()));

        let mut rx = tx.subscribe();
        //println!("Starting device {} message loop", self.name());
        _ = tx.send(ev);

        let dev = self.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(ev) => dev.handle_event(ev, &tx),
                    Err(err) => println!("error receiving event in device: {}", err),
                }
            }
        });
    }

    fn handle_event(&self, ev: Event, _tx: &Sender) {
        if let Some(src) = ev.source {
            if src.name() == self.name() {
                return; // ignore own events
            }
        }
        println!("{}: {}", self.name(), ev.name);
    }

    pub fn name(&self) -> String {
        let d = self.value.lock().unwrap();
        d.name.clone()
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
