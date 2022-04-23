use crate::{
    device::DeviceInterface,
    event::{Event, Sender},
};
use std::{thread, time::Duration};
use termion::{color, style};

#[derive(Clone, PartialEq)]
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

impl DeviceInterface for DummyDevice {
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

    fn on_event(&mut self, ev: Event, _tx: &Sender) {
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
}
