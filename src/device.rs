use std::sync::{Arc, Mutex};

use yaml_rust::Yaml;

use crate::event::{EventHandler, Sender};
use crate::result::RHomeResult;
use crate::Ptr;

pub trait Device: Send + EventHandler {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn configure(&mut self, _configuration: &Yaml) -> RHomeResult<()> {
        Ok(())
    }
    fn start(&mut self, _tx: Sender) -> bool {
        return true;
    }
    fn stop(&mut self) {}
}

pub(crate) type DevicePtr = Ptr<dyn Device>;
