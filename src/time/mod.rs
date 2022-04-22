use std::{thread, time::Duration};

use uuid::Uuid;

use crate::{
    device::DeviceInterface,
    event::{Event, Sender},
    types::RHomeObject,
};

#[derive(Clone, PartialEq)]
pub struct TimeDevice {
    id: Uuid,
    name: String,
}

impl RHomeObject for TimeDevice {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl DeviceInterface for TimeDevice {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn start(&mut self, tx: Sender) -> bool {
        println!("starting time");
        let ev = Event::new(format!("{} started", self.get_name()), self.id());
        let tx_for_thread = tx.clone();
        let my_id = self.id();
        _ = tx.send(ev);

        thread::spawn(move || {
            for i in 1..10 {
                //                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(3));
                let ev = Event::new(format!("number {} time thread!", i), my_id);
                _ = tx_for_thread.send(ev);
            }
        });

        true
    }
}

impl TimeDevice {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "".to_string(),
        }
    }
}

// impl DeviceInterface for TimeDevice {}

// impl RHomeObject for TimeDevice {
//     fn id(&self) -> Uuid {
//         let d = self.value.lock().unwrap().as_ref();
//         d.id()
//     }
// }