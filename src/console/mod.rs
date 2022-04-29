use crate::driver::Driver;
use device::ConsoleDevice;
use std::sync::{Arc, Mutex};

mod device;

pub(crate) struct ConsoleDriver {}

impl ConsoleDriver {
    pub(crate) fn new() -> Box<dyn Driver> {
        Box::new(Self {})
    }
}

impl Driver for ConsoleDriver {
    fn load(
        &mut self,
        configuration: &yaml_rust::Yaml,
        manager: &mut dyn crate::Manager,
    ) -> crate::result::RHomeResult<()> {
        let name = configuration["name"]
            .as_str()
            .unwrap_or("console")
            .to_string();

        manager.add_device(name, Arc::new(Mutex::new(Box::new(ConsoleDevice::new()))));
        Ok(())
    }
}
