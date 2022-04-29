use device::IDevice;
use driver::IDriver;
use event::{run_event_loop, EventHandler};
use manager::manager;
use result::RHomeResult;
use std::sync::{Arc, Mutex};

pub(crate) mod device;
pub(crate) mod driver;
mod dummy;
mod event;
pub(crate) mod manager;
mod result;
mod time;

// pub(crate) struct Ptr<T> {
//     obj: Arc<Mutex<T>>,
// }

// trait Ptr<ObjType = Self> {
//     fn value(&self) -> Arc<Mutex<ObjType>>;

//     fn lock(&self) -> Result<MutexGuard<ObjType>, PoisonError<MutexGuard<ObjType>>> {
//         self.value().lock()
//     }
// }

// #[derive(Clone)]
// pub(crate) struct Manager {
//     obj: Arc<Mutex<dyn IManager>>,
// }

// impl Manager {
//     // pub fn lock(&self) -> Result<MutexGuard<dyn IManager>, PoisonError<MutexGuard<dyn IManager>>> {
//     //     self.obj.lock()
//     // }

//     async fn start(&mut self) {
//         //let tx: Sender;
//         let mut rx: Receiver;
//         {
//             let mngr = self.obj.lock().unwrap();
//             rx = mngr.receiver();
//             //tx = mngr.tx.clone();
//         }

//         //println!("Starting manager message loop");
//         //        let mut rx = tx.subscribe();
//         loop {
//             match rx.recv().await {
//                 Ok(ev) => {
//                     let mngr = self.clone();
//                     tokio::spawn(async move {
//                         let mngr = mngr.obj.lock().unwrap();
//                         mngr.handle_event(ev);
//                     });
//                 }
//                 Err(err) => println!("error receiving event in manager: {}", err),
//             }
//         }
//     }

//     pub(crate) fn load_driver(&mut self, name: String, driver: Driver, configuration: &Yaml) {
//         // {
//         //     match driver.configure(configuration) {
//         //         Ok(_) => (),
//         //         Err(err) => println!("error configuring driver {}: {}", &name, err),
//         //     }
//         // }

//         let (t1, r1) = channel();
//         let mngr = self.obj.lock().unwrap();
//         {
//             let mut drivers = mngr.drivers.lock().unwrap();
//             drivers.insert(
//                 name,
//                 DriverInfo {
//                     // device: device,
//                     tx: t1,
//                 },
//             );
//         }
//         let mut driver = driver.obj.lock().unwrap();
//         {
//             //let tx = self.tx.clone();
//             //            let tx2 = tx.clone();
//             _ = driver.load(configuration);

//             // let ev = Event::new("loaded".to_string(), n);
//             // //let tx_for_thread = tx.clone();
//             // _ = tx2.send(ev);
//         };
//     }
// }

// impl Ptr for Manager {
//     fn value(&self) -> Arc<Mutex<Self>> {
//         self.obj
//     }
// }

pub(crate) trait Manager: EventHandler {
    fn load_drivers(&mut self, config_file: String) -> RHomeResult<()>;
    fn load_driver(
        &mut self,
        name: String,
        driver: Arc<Mutex<Box<dyn IDriver>>>,
    ) -> RHomeResult<()>;
    fn add_device(&mut self, name: String, device: Arc<Mutex<Box<dyn IDevice>>>);
}

// pub(crate) struct Ptr<T: ?Sized> {
//     value: Arc<Mutex<T>>,
// }

// impl<T> Ptr<T> {
//     // pub(crate) fn new(obj: &mut T) -> Self {
//     //     Self {
//     //         value: Arc::new(Mutex::new(obj)),
//     //     }
//     // }

//     pub fn lock(&self) -> Result<MutexGuard<T>, PoisonError<MutexGuard<T>>> {
//         self.value.lock()
//     }
// }

#[tokio::main]
async fn main() {
    {
        let manager = manager();

        // manager.add(
        //     "time".to_string(),
        //     Device::new(Box::new(time::TimeDevice::new())),
        // );

        {
            let mngr = manager.clone();
            tokio::spawn(async move {
                {
                    // if let Err(err) = mngr.add_devices_from_config("config.yaml".to_string()) {
                    //     println!("unable to crate devices from configuration file: {:?}", err)
                    // }
                    let mut m = mngr.lock().unwrap();
                    if let Err(err) = m.load_drivers("config.yaml".to_string()) {
                        println!("unable to load drivers from configuration file: {:?}", err)
                    }
                };

                // let d = Device::new(Box::new(dummy::DummyDevice::new("light01".to_string())));
                // sleep(Duration::from_millis(1000)).await;
                // mngr.add("light1".to_string(), d.clone());
            });
        }

        //println!("Starting manager");
        // {
        //     let mngr = manager.clone();
        // }
        run_event_loop(manager).await;
    }
}
