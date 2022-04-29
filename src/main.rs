use device::DevicePtr;
use driver::DriverPtr;
use event::{run_event_loop, EventHandler, EventSender, Receiver};
use manager::manager;
use result::RHomeResult;
use std::sync::{Arc, Mutex};

mod console;
pub(crate) mod device;
pub(crate) mod driver;
mod dummy;
mod event;
pub(crate) mod manager;
mod result;
mod time;

pub(crate) trait Manager: EventHandler + EventSender + Send {
    fn load_drivers(&mut self, config_file: String) -> RHomeResult<()>;
    fn load_driver(&mut self, name: String, driver: DriverPtr) -> RHomeResult<()>;
    fn add_device(&mut self, name: String, device: DevicePtr);
}

pub(crate) type ManagerPtr = Ptr<dyn Manager>;

#[tokio::main]
async fn main() {
    {
        let manager = manager();

        let rx: Receiver;
        {
            let mngr = manager.clone();
            tokio::spawn(async move {
                let mut m = mngr.lock().unwrap();
                if let Err(err) = m.load_drivers("config.yaml".to_string()) {
                    println!("unable to load drivers from configuration file: {:?}", err)
                }
            });
        }
        {
            let mngr = manager.clone();
            let m = mngr.lock().unwrap();
            rx = m.get_receiver();
        }
        run_event_loop(rx, manager).await;
    }
}

type Ptr<T> = Arc<Mutex<Box<T>>>;
