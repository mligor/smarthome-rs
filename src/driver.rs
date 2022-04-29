use std::sync::{Arc, Mutex};

use crate::{result::RHomeResult, Manager, Ptr};
use yaml_rust::Yaml;

pub(crate) trait Driver: Send {
    fn load(&mut self, configuration: &Yaml, manager: &mut dyn Manager) -> RHomeResult<()>;
}

pub(crate) type DriverPtr = Ptr<dyn Driver>;
