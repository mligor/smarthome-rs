use yaml_rust::Yaml;

use crate::event::{Event, Receiver, Sender};
use crate::result::Result;
use std::sync::{Arc, Mutex};

pub trait DeviceInterface: Send {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn configure(&mut self, _configuration: &Yaml) -> Result<()> {
        Ok(())
    }
    fn start(&mut self, _tx: Sender) -> bool {
        return true;
    }
    fn stop(&mut self) {}
    fn on_event(&mut self, _ev: &Event) {}
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

    pub fn start(&mut self, tx: Sender, mut rx: Receiver) {
        let ev = Event::new("start".to_string(), self.name());

        //let mut rx = tx.subscribe();
        //println!("Starting device {} message loop", self.name());
        _ = tx.send(ev);

        let mut dev = self.clone();
        let tx2 = tx.clone();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(ev) => dev.handle_event(&ev),
                    Err(err) => println!("error receiving event in device: {}", err),
                }
            }
        });
        self.value.lock().unwrap().start(tx2);
    }

    fn handle_event(&mut self, ev: &Event) {
        self.value.lock().unwrap().on_event(ev);
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.value.lock().unwrap().set_name(name);
    }

    pub(crate) fn name(&self) -> String {
        self.value.lock().unwrap().name()
    }

    pub(crate) fn configure(&mut self, configuration: &Yaml) -> Result<()> {
        self.value.lock().unwrap().configure(configuration)
    }
}
