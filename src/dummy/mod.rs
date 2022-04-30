use crate::{
    device::{Device, DevicePtr},
    driver::Driver,
    event::{Event, EventHandler},
    result::RHomeResult,
};
use termion::{color, style};

pub struct DummyDevice {
    name: String,
}

impl DummyDevice {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl EventHandler for DummyDevice {
    fn handle_event(&mut self, ev: Event) {
        println!(
            "{}{}{}{} : {}{}",
            color::Fg(color::Cyan),
            style::Bold,
            self.name,
            style::Reset,
            ev,
            style::Reset
        );
    }
}

impl Device for DummyDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self, _tx: crate::event::Sender) -> RHomeResult<()> {
        Ok(())
    }
}

pub struct DummyDriver {}

impl DummyDriver {
    pub(crate) fn new() -> Box<dyn Driver> {
        Box::new(Self {})
    }
}

impl Driver for DummyDriver {
    fn load(
        &mut self,
        _configuration: &yaml_rust::Yaml,
        manager: &mut dyn crate::Manager,
    ) -> crate::result::RHomeResult<()> {
        manager.add_device(DevicePtr::new(Box::new(DummyDevice::new(
            "dummy_watcher".to_string(),
        ))))?;
        Ok(())
    }
}
