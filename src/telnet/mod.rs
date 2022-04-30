use crate::{device::DevicePtr, driver::Driver, result::RHomeResult, Manager};
use device::TelnetDevice;
use yaml_rust::Yaml;

mod device;

pub struct TelnetDriver {}

impl TelnetDriver {
    pub(crate) fn new() -> Box<dyn Driver> {
        Box::new(Self {})
    }
}

impl Driver for TelnetDriver {
    fn load(&mut self, configuration: &Yaml, manager: &mut dyn Manager) -> RHomeResult<()> {
        let name = configuration["name"]
            .as_str()
            .unwrap_or("console")
            .to_string();

        let listen_on: String = configuration["listen_on"]
            .as_str()
            .unwrap_or("127.0.0.1:7800")
            .to_string();

        manager.add_device(DevicePtr::new(Box::new(TelnetDevice::new(name, listen_on))))?;
        Ok(())
    }
}
