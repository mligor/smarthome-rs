use crate::event::{EventHandler, Sender};
use crate::result::RHomeResult;
use crate::Ptr;
use yaml_rust::Yaml;

pub(crate) trait Device: Send + EventHandler {
    fn name(&self) -> String;
    fn configure(&mut self, _configuration: &Yaml) -> RHomeResult<()> {
        Ok(())
    }
    fn start(&mut self, _tx: Sender) -> RHomeResult<()>;
    fn stop(&mut self) {}
}

pub(crate) type DevicePtr = Ptr<dyn Device>;
