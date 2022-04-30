use termion::{color, style};

use crate::{
    device::Device,
    event::{Event, EventHandler, Sender},
    result::RHomeResult,
};

#[derive(Default)]
pub struct ConsoleDevice {
    name: String,
}

impl ConsoleDevice {
    pub fn new() -> Self {
        ConsoleDevice::default()
    }
}

impl EventHandler for ConsoleDevice {
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

impl Device for ConsoleDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn start(&mut self, _tx: Sender) -> RHomeResult<()> {
        Ok(())
    }
}
