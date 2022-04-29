use crate::{
    device::Device,
    driver::Driver,
    event::{Event, EventHandler},
};
use std::sync::{Arc, Mutex};
use termion::{color, style};

#[derive(Default)]
pub struct DummyDevice {
    name: String,
}

impl DummyDevice {
    pub fn new() -> Self {
        DummyDevice::default()
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

    fn set_name(&mut self, name: String) {
        self.name = name
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
        manager.add_device(
            "dummy_watcher".to_string(),
            Arc::new(Mutex::new(Box::new(DummyDevice::new()))),
        );
        Ok(())
    }
}
