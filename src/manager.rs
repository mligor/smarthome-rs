use crate::{
    device::IDevice,
    driver::IDriver,
    dummy,
    event::{channel, Event, EventHandler, Receiver, Sender},
    result::{RHomeError, RHomeResult},
    time, Manager,
};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};
use termion::{color, style};
use yaml_rust::{Yaml, YamlLoader};

type DeviceList = Arc<Mutex<HashMap<String, DeviceInfo>>>;
type DriverList = Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn IDriver>>>>>>;

struct DeviceInfo {
    //device: Device,
    tx: Sender,
}

#[derive(Clone)]
pub(crate) struct ManagerImpl {
    tx: Sender,
    devices: DeviceList,
    drivers: DriverList,
}

pub(crate) fn manager() -> Arc<Mutex<Box<impl Manager>>> {
    Arc::new(Mutex::new(Box::new(ManagerImpl::new())))
}

impl ManagerImpl {
    pub fn new() -> Self {
        let (tx, _) = channel();

        Self {
            tx,
            devices: Arc::new(Mutex::new(HashMap::new())),
            drivers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl EventHandler for ManagerImpl {
    fn handle_event(&mut self, ev: Event) {
        if "manager" == ev.source {
            return; // Ignore own events
        }

        println!(
            "{}{}{}{} : {}{}",
            color::Fg(color::Green),
            style::Bold,
            "manager",
            style::Reset,
            ev,
            style::Reset
        );

        // Broadcast to all devices
        let mut senders: Vec<Sender> = Vec::new();
        {
            let mut devices = self.devices.lock().unwrap();

            for (dev_name, dev) in devices.iter_mut() {
                if *dev_name == ev.source {
                    continue; // ignore own events
                }
                senders.push(dev.tx.clone());
            }
        }

        // Ensure that nothing is locked during sent
        for sender in senders {
            _ = sender.send(ev.clone());
        }
    }

    fn create_receiver(&mut self) -> Receiver {
        self.tx.subscribe()
    }
}

impl Manager for ManagerImpl {
    fn load_drivers(&mut self, config_file: String) -> RHomeResult<()> {
        let content = fs::read_to_string(config_file)?;

        let docs = YamlLoader::load_from_str(&content)?;
        let doc = &docs[0]; // Take only first document (for now)
        let drivers = &doc["drivers"].as_hash();
        if None == drivers.as_ref() {
            return Ok(());
        }
        let drivers = drivers.unwrap();

        for (driver_name, driver_config) in drivers.iter() {
            let driver_name = driver_name.as_str().unwrap().to_string();

            match create_driver(&driver_name, driver_config, self) {
                Ok(driver) => match self.load_driver(driver_name.clone(), driver) {
                    Ok(_) => {}
                    Err(err) => println!(
                        "{}{}error: unable to load driver '{:?}' : {:?}{}",
                        color::Fg(color::Red),
                        style::Bold,
                        driver_name,
                        err.to_string(),
                        style::Reset
                    ),
                },
                Err(err) => println!(
                    "{}{}error: unable to create driver '{:?}' : {:?}{}",
                    color::Fg(color::Red),
                    style::Bold,
                    driver_name,
                    err.to_string(),
                    style::Reset
                ),
            }
        }

        Ok(())
    }

    fn load_driver(
        &mut self,
        name: String,
        driver: Arc<Mutex<Box<dyn IDriver>>>,
    ) -> RHomeResult<()> {
        let mut drivers = self.drivers.lock().unwrap();
        drivers.insert(name, driver);
        Ok(())
    }

    fn add_device(&mut self, name: String, device: Arc<Mutex<Box<dyn IDevice>>>) {
        let mut devices = self.devices.lock().unwrap();
        let (tx, rx) = channel();
        {
            let mut dev = device.lock().unwrap();
            dev.set_receiver(rx);
            dev.set_name(name.clone());
        }
        devices.insert(name, DeviceInfo { tx: tx });
        {
            let mut dev = device.lock().unwrap();
            let tx2 = self.tx.clone();
            dev.start(tx2);
        }
    }
}

impl ManagerImpl {
    //     pub(crate) fn new() -> ManagerImpl {
    //         let (tx, _) = channel();
    //         ManagerImpl {
    //             value: Arc::new(Mutex::new(DeviceManagerData { tx })),
    //             devices: Arc::new(Mutex::new(HashMap::new())),
    //             drivers: Arc::new(Mutex::new(HashMap::new())),
    //         }
    //     }
}

//     pub(crate) async fn start(&mut self) {
//         //let tx: Sender;
//         let mut rx: Receiver;
//         {
//             let mngr = self.value.lock().unwrap();
//             rx = mngr.tx.subscribe();
//             //tx = mngr.tx.clone();
//         }

//         //println!("Starting manager message loop");
//         //        let mut rx = tx.subscribe();
//         loop {
//             match rx.recv().await {
//                 Ok(ev) => {
//                     let mngr = self.clone();
//                     tokio::spawn(async move {
//                         mngr.handle_event(ev);
//                     });
//                 }
//                 Err(err) => println!("error receiving event in manager: {}", err),
//             }
//         }
//     }

//     pub(crate) fn add_devices_from_config(&mut self, config_file: String) -> RHomeResult<()> {
//         let content = fs::read_to_string(config_file)?;

//         let docs = YamlLoader::load_from_str(&content)?;
//         let doc = &docs[0]; // Take only first document (for now)
//         let devices = &doc["devices"].as_hash();
//         if None == devices.as_ref() {
//             return Ok(());
//         }
//         let devices = devices.unwrap();

//         for (device_name, device) in devices.iter() {
//             let device_name = device_name.as_str().unwrap().to_string();
//             let driver = device["driver"].as_str();
//             if None == driver {
//                 eprintln!(
//                     "unable to create device '{}' : driver is not specified",
//                     device_name,
//                 );
//                 continue;
//             }
//             let driver = driver.unwrap();

//             //println!("device = {:?} / {:?}", device_name, device_type);
//             match create_device(driver) {
//                 Ok(dev) => {
//                     self.add_device(device_name.to_string(), dev, device);
//                 }
//                 Err(err) => {
//                     println!(
//                         "{}{}error: unable to create device '{:?}' : {:?}{}",
//                         color::Fg(color::Red),
//                         style::Bold,
//                         device_name,
//                         err.to_string(),
//                         style::Reset
//                     );
//                     // eprintln!(
//                     //     "unable to create device '{:?}' : {:?}",
//                     //     device_name,
//                     //     err.to_string(),
//                     // );
//                 }
//             }
//         }
//         Ok(())
//     }

//     pub(crate) fn load_drivers_from_config(&mut self, config_file: String) -> RHomeResult<()> {
//         let content = fs::read_to_string(config_file)?;

//         let docs = YamlLoader::load_from_str(&content)?;
//         let doc = &docs[0]; // Take only first document (for now)
//         let drivers = &doc["drivers"].as_hash();
//         if None == drivers.as_ref() {
//             return Ok(());
//         }
//         let drivers = drivers.unwrap();

//         for (driver_name, driver_config) in drivers.iter() {
//             let driver_name = driver_name.as_str().unwrap().to_string();

//             match create_driver(&driver_name) {
//                 Ok(driver) => self.load_driver(driver_name, driver, driver_config),
//                 Err(err) => {
//                     println!(
//                         "{}{}error: unable to create driver '{:?}' : {:?}{}",
//                         color::Fg(color::Red),
//                         style::Bold,
//                         driver_name,
//                         err.to_string(),
//                         style::Reset
//                     );
//                     // eprintln!(
//                     //     "unable to create device '{:?}' : {:?}",
//                     //     device_name,
//                     //     err.to_string(),
//                     // );
//                 }
//             }
//         }

//         Ok(())
//     }

//     fn handle_event(&self, ev: Event) {
//         if "manager" == ev.source {
//             return; // Ignore own events
//         }

//         println!(
//             "{}{}{}{} : {}{}",
//             color::Fg(color::Green),
//             style::Bold,
//             "manager",
//             style::Reset,
//             ev,
//             style::Reset
//         );

//         // Broadcast to all devices
//         let mut senders: Vec<Sender> = Vec::new();
//         {
//             let mut devices = self.devices.lock().unwrap();

//             for (dev_name, dev) in devices.iter_mut() {
//                 if *dev_name == ev.source {
//                     continue; // ignore own events
//                 }
//                 senders.push(dev.tx.clone());
//             }
//         }

//         // Ensure that nothing is locked during sent
//         for sender in senders {
//             _ = sender.send(ev.clone());
//         }
//     }
// }

// fn create_device(driver: &str) -> RHomeResult<Device> {
//     match driver {
//         "time" => Ok(Device::new(Box::new(TimeDevice::new()))),
//         "dummy" => Ok(Device::new(Box::new(DummyDevice::new()))),
//         _ => Err(RHomeError::new(format!(
//             "unknown device driver '{}'",
//             driver
//         ))),
//     }
// }

fn create_driver(
    name: &str,
    configuration: &Yaml,
    manager: &mut dyn Manager,
) -> RHomeResult<Arc<Mutex<Box<dyn IDriver>>>> {
    let mut d: Box<dyn IDriver> = match name {
        "time" => Ok(time::driver::Driver::new()),
        "dummy" => Ok(dummy::Driver::new()),
        _ => Err(RHomeError::new(format!("unknown driver '{}'", name))),
    }?;
    d.load(configuration, manager)?;
    Ok(Arc::new(Mutex::new(d)))
}

// impl Manager for ManagerImpl {
//     fn add_device(&mut self, name: String, device: Device, configuration: &Yaml) {
//         let n = name.clone();
//         let dev_copy = device.clone();
//         {
//             let mut device = device.clone();
//             device.set_name(name.clone());
//             match device.configure(configuration) {
//                 Ok(_) => (),
//                 Err(err) => println!("error configuring device {}: {}", &name, err),
//             }
//         }
//         let (t1, r1) = channel();
//         let mngr = self.value.lock().unwrap();
//         {
//             let mut devices = self.devices.lock().unwrap();
//             devices.insert(
//                 name,
//                 DeviceInfo {
//                     // device: device,
//                     tx: t1,
//                 },
//             );
//         }
//         // if mngr.started
//         {
//             let tx = mngr.tx.clone();
//             let tx2 = tx.clone();
//             dev_copy.clone().start(tx, r1);

//             let ev = Event::new("start".to_string(), n);
//             //let tx_for_thread = tx.clone();
//             _ = tx2.send(ev);
//         };
//     }
// }
