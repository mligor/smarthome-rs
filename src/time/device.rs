use crate::{
    device::Device,
    event::{Event, EventDataValue, EventHandler, Sender},
    result::RHomeResult,
};
use chrono::{Local, Timelike, Utc};
use std::{collections::HashMap, thread, time::Duration};

pub(crate) struct TimeDevice {
    name: String,
    format: String,
    local_time: bool,
    every_second: bool,
}
impl EventHandler for TimeDevice {}

impl Device for TimeDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn configure(&mut self, configuration: &yaml_rust::Yaml) -> crate::result::RHomeResult<()> {
        self.format = configuration["format"]
            .as_str()
            .unwrap_or(&self.format)
            .to_string();

        self.local_time = configuration["local_time"]
            .as_bool()
            .unwrap_or(self.local_time);

        self.every_second = configuration["every_second"]
            .as_bool()
            .unwrap_or(self.every_second);

        Ok(())
    }

    fn start(&mut self, tx: Sender) -> RHomeResult<()> {
        let my_name = self.name();
        let format = self.format.clone();
        let local_time = self.local_time;
        let every_second = self.every_second;
        thread::spawn(move || loop {
            let now = Utc::now();
            let ns = now.nanosecond();
            let delay: u64;

            if every_second {
                delay = u64::from(1000000000 - ns)
            } else {
                let seconds = now.second();
                delay = u64::from(1000000000 - ns) + (1000000000 * (59 - u64::from(seconds)));
            }
            thread::sleep(Duration::from_nanos(delay));
            let now2 = Utc::now();
            let time: String;
            if !local_time {
                time = now2.format(&format).to_string();
            } else {
                let now2 = now2.with_timezone(&Local);
                time = now2.format(&format).to_string();
            }

            let mut ev = Event::new("current_time".to_string(), my_name.clone());
            ev.data = HashMap::from([("time", EventDataValue::String(time))]);
            _ = tx.send(ev);
        });

        Ok(())
    }
}

impl TimeDevice {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            format: "%+".to_string(),
            local_time: false,
            every_second: false,
        }
    }
}
