use crate::{
    device::DeviceInterface,
    event::{Event, Sender},
};
use chrono::{Timelike, Utc};
use std::{collections::HashMap, thread, time::Duration};

#[derive(Clone, PartialEq)]
pub struct TimeDevice {
    name: String,
}

impl DeviceInterface for TimeDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn start(&mut self, tx: Sender) -> bool {
        let my_name = self.name();
        thread::spawn(move || loop {
            let now = Utc::now();
            let ns = now.nanosecond();
            let seconds = now.second();
            let delay = u64::from(1000000000 - ns) + (1000000000 * (59 - u64::from(seconds)));
            thread::sleep(Duration::from_nanos(delay));
            let now2 = Utc::now();
            let mut ev = Event::new("current_time".to_string(), my_name.clone());
            ev.data = HashMap::from([("time".to_string(), now2.to_string())]);
            _ = tx.send(ev);
        });

        true
    }
}

impl TimeDevice {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}
