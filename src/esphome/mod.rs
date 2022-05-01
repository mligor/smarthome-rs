use crate::{device::DevicePtr, driver::Driver, result::RHomeResult, Manager};
use device::EspHomeDevice;
use yaml_rust::Yaml;

mod device;

pub struct EspHomeDriver {}

impl EspHomeDriver {
    pub(crate) fn new() -> Box<dyn Driver> {
        Box::new(Self {})
    }
}

impl Driver for EspHomeDriver {
    fn load(&mut self, configuration: &Yaml, manager: &mut dyn Manager) -> RHomeResult<()> {
        if let Some(connections) = configuration["connections"].as_vec() {
            for con in connections {
                let name = con["name"].as_str().unwrap_or("").to_string();
                let host = con["host"].as_str().unwrap_or("").to_string();
                let password = con["password"].as_str().unwrap_or("").to_string();
                if host != "" && name != "" {
                    manager.add_device(DevicePtr::new(Box::new(EspHomeDevice::new(
                        name, host, password,
                    ))))?;
                }
            }
        }
        Ok(())
    }
}
