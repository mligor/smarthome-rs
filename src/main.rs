use device::DevicePtr;
use driver::DriverPtr;
use event::{run_event_loop, EventHandler, EventSender, Receiver};
use manager::manager;
use result::RHomeResult;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

pub(crate) mod device;
pub(crate) mod driver;
pub(crate) mod event;
pub(crate) mod manager;
pub(crate) mod result;

mod dummy;
mod telnet;
mod time;

pub(crate) trait Manager: EventHandler + EventSender + Send {
    fn load_drivers(&mut self, config_file: String) -> RHomeResult<()>;
    fn load_driver(&mut self, name: String, driver: DriverPtr) -> RHomeResult<()>;
    fn add_device(&mut self, device: DevicePtr) -> RHomeResult<()>;
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

pub(crate) struct Ptr<T>
where
    T: ?Sized,
{
    value: Arc<Mutex<Box<T>>>,
}

impl<T> Ptr<T>
where
    T: ?Sized,
{
    pub(crate) fn new(d: Box<T>) -> Self {
        Self {
            value: Arc::new(Mutex::new(d)),
        }
    }

    pub(crate) fn lock(&self) -> Result<MutexGuard<Box<T>>, PoisonError<MutexGuard<Box<T>>>> {
        self.value.lock()
    }
}

impl<T> Clone for Ptr<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

// struct Ptr<T> {
//     value: Arc<Mutex<Box<T>>>,
// }
