use crate::event::{Event, Sender};
use std::sync::{Arc, Mutex};

pub trait DeviceInterface: Send {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn start(&mut self, _tx: Sender) -> bool {
        return true;
    }
    fn stop(&mut self) {}
    fn on_event(&mut self, _ev: Event, _tx: &Sender) {}
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
        let ev = Event::new("start".to_string(), self.name());

        let mut rx = tx.subscribe();
        println!("Starting device {} message loop", self.name());
        _ = tx.send(ev);
        let mut dev = self.clone();
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

    fn handle_event(&mut self, ev: Event, tx: &Sender) {
        if self.name() == ev.source {
            return; // ignore own events
        }
        self.value.lock().unwrap().on_event(ev, tx);
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.value.lock().unwrap().set_name(name);
    }

    pub(crate) fn name(&self) -> String {
        self.value.lock().unwrap().name()
    }
}
