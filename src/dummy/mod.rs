use crate::{
    device::Device,
    driver::Driver,
    event::{Event, EventHandler, Sender},
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use termion::{color, style};

//#[derive(Clone, PartialEq)]
pub struct DummyDevice {
    name: String,
}

impl DummyDevice {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}

impl EventHandler for DummyDevice {
    fn handle_event(&mut self, ev: Event) {
        let name = self.name.clone();

        println!(
            "{}{}{}{} : {}{}",
            color::Fg(color::Cyan),
            style::Bold,
            name,
            style::Reset,
            ev,
            style::Reset
        );
    }

    // fn create_receiver(&mut self) -> Receiver {}
}

impl Device for DummyDevice {
    // fn set_receiver(&mut self, rx: crate::event::Receiver) {
    //     self.rx = Some(rx);
    // }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn start(&mut self, tx: Sender) -> bool {
        let my_name = self.name();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let ev = Event::new("test".to_string(), my_name.clone());
            _ = tx.send(ev);
        });
        true
    }

    fn configure(&mut self, _configuration: &yaml_rust::Yaml) -> crate::result::RHomeResult<()> {
        Ok(())
    }

    fn stop(&mut self) {}
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
            "d1".to_string(),
            Arc::new(Mutex::new(Box::new(DummyDevice::new()))),
        );
        Ok(())
    }

    // fn add_device(
    //     &mut self,
    //     name: String,
    //     device: crate::device::Device,
    //     configuration: &yaml_rust::Yaml,
    // ) {
    // }
}
