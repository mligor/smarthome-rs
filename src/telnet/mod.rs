use crate::{device::Device, driver::Driver, Manager};
use device::ConsoleDevice;
use std::sync::{Arc, Mutex};

mod device;

pub(crate) struct ConsoleDriver<D, R, M>
where
    R: Driver<M>,
    M: Manager<D, R>,
    D: Device, {}

impl<D, R, M> ConsoleDriver<D, R, M>
where
    R: Driver<M>,
    M: Manager<D, R>,
    D: Device,
{
    pub(crate) fn new() -> Box<R> {
        Box::new(Self {})
    }
}

impl<D, R, M> Driver<M> for ConsoleDriver<D, R, M>
where
    D: Device,
    R: Driver<M>,
    M: Manager<D, R>,
{
    fn initialize(
        &mut self,
        configuration: &yaml_rust::Yaml,
        manager: Arc<Mutex<Box<M>>>,
    ) -> crate::result::RHomeResult<()> {
        let name = configuration["name"]
            .as_str()
            .unwrap_or("console")
            .to_string();

        let listen_on: String = configuration["listen_on"]
            .as_str()
            .unwrap_or("127.0.0.1:7800")
            .to_string();

        manager.add_device(Arc::new(Mutex::new(Box::new(ConsoleDevice::new(
            name, listen_on,
        )))));
        Ok(())
    }
}
