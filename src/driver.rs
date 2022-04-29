use crate::{result::RHomeResult, Manager};
use yaml_rust::Yaml;

pub(crate) trait IDriver: Send {
    fn load(&mut self, configuration: &Yaml, manager: &mut dyn Manager) -> RHomeResult<()>;
    //    fn add_device(&mut self, name: String, device: Device, configuration: &Yaml);
}

// pub(crate) trait DriverInterface: Send {
//     fn name(&self) -> String;
//     fn configure(&mut self, _configuration: &Yaml) -> RHomeResult<()> {
//         Ok(())
//     }
//     fn load(&mut self, _mngr: Arc<Mutex<dyn Manager>>, _tx: Sender, _rx: Receiver) -> bool {
//         return true;
//     }
//     fn unload(&mut self) {}
//     fn create_device(&mut self, _name: String) -> RHomeResult<Device> {
//         Err(RHomeError::new("not supported".to_string()))
//     }
// }
