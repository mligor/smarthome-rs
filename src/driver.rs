use crate::{result::RHomeResult, Manager};
use yaml_rust::Yaml;

pub(crate) trait Driver: Send {
    fn load(&mut self, configuration: &Yaml, manager: &mut dyn Manager) -> RHomeResult<()>;
}
