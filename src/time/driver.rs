use std::sync::{Arc, Mutex};

use crate::device::Device;
use crate::driver::Driver;
use crate::result::RHomeResult;
use crate::Manager;
use yaml_rust::Yaml;

use super::device::TimeDevice;

#[derive(Clone)]
pub struct TimeDriver {
    format: String,
    local_time: bool,
    every_second: bool,
}

impl TimeDriver {
    pub(crate) fn new() -> Box<dyn Driver> {
        Box::new(Self {
            format: "%+".to_string(),
            local_time: false,
            every_second: false,
        })
    }
}

impl Driver for TimeDriver {
    fn load(&mut self, configuration: &Yaml, manager: &mut dyn Manager) -> RHomeResult<()> {
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

        let mut t = TimeDevice::new();
        t.configure(configuration).unwrap();

        manager.add_device("time".to_string(), Arc::new(Mutex::new(Box::new(t))));
        Ok(())
    }
}
