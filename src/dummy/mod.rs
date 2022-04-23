use std::{thread, time::Duration};

use crate::{
    device::DeviceInterface,
    event::{Event, Sender},
    types::RHomeObject,
};
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct DummyDevice {
    id: Uuid,
    name: String,
}

impl DummyDevice {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_string(),
        }
    }
}

impl RHomeObject for DummyDevice {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl DeviceInterface for DummyDevice {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn start(&mut self, tx: Sender) -> bool {
        let my_id = self.id();
        let my_name = self.get_name();
        let ev = Event::new(
            format!("{} started", self.get_name()),
            my_id,
            my_name.clone(),
        );
        let tx_for_thread = tx.clone();
        _ = tx.send(ev);

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            let ev = Event::new(format!("event from {}", my_name), my_id, my_name.clone());
            _ = tx_for_thread.send(ev);
        });

        true
    }

    fn stop(&mut self) {}
}

// impl DeviceInterface for DummyDevice {
//     // fn get_type(&self) -> String {
//     //     "dummy".to_string()
//     // }
// }
