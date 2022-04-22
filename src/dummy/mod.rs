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
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
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
        println!("starting dummy");
        let ev = Event::new(format!("{} started", self.get_name()), self.id());
        let tx_for_thread = tx.clone();
        let my_id = self.id();
        _ = tx.send(ev);

        thread::spawn(move || {
            for i in 1..10 {
                //                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(3));
                let ev = Event::new(format!("number {} time from dummy!", i), my_id);
                _ = tx_for_thread.send(ev);
            }
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
