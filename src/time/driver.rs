use std::sync::{Arc, Mutex};

use crate::device::IDevice;
use crate::driver::IDriver;
use crate::result::RHomeResult;
use crate::Manager;
use yaml_rust::Yaml;

use super::device::TimeDevice;

#[derive(Clone)]
pub struct Driver {
    format: String,
    local_time: bool,
    every_second: bool,
}

impl Driver {
    pub(crate) fn new() -> Box<dyn IDriver> {
        Box::new(Self {
            format: "%+".to_string(),
            local_time: false,
            every_second: false,
        })
    }
}

impl IDriver for Driver {
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

    // fn add_device(&mut self, name: String, device: Device, configuration: &Yaml) {}
}

// impl DriverInterface for Driver {
//     fn name(&self) -> String {
//         "time".to_string()
//     }

//     fn configure(&mut self, configuration: &Yaml) -> RHomeResult<()> {
//         self.format = configuration["format"]
//             .as_str()
//             .unwrap_or(&self.format)
//             .to_string();

//         self.local_time = configuration["local_time"]
//             .as_bool()
//             .unwrap_or(self.local_time);

//         self.every_second = configuration["every_second"]
//             .as_bool()
//             .unwrap_or(self.every_second);

//         Ok(())
//     }

//     fn load(
//         &mut self,
//         _mngr: std::sync::Arc<std::sync::Mutex<dyn crate::Manager>>,
//         _tx: crate::event::Sender,
//         _rx: crate::event::Receiver,
//     ) -> bool {
//         return true;
//     }

//     //    fn unload(&mut self) {}

//     fn create_device(&mut self, name: String) -> RHomeResult<Device> {
//         Err(crate::result::RHomeError::new("not supported".to_string()))
//     }
// }
