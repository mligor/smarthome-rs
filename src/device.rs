use yaml_rust::Yaml;

use crate::event::{EventHandler, Sender};
use crate::result::RHomeResult;

pub trait Device: Send + EventHandler {
    //    fn set_receiver(&mut self, rx: Receiver);
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn configure(&mut self, _configuration: &Yaml) -> RHomeResult<()> {
        Ok(())
    }
    fn start(&mut self, _tx: Sender) -> bool {
        return true;
    }
    fn stop(&mut self) {}
    //    fn on_event(&mut self, _ev: &Event) {}
}
