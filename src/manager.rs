use crate::{
    device::DevicePtr,
    driver::{Driver, DriverPtr},
    dummy::DummyDriver,
    event::{
        channel, run_event_loop, Event, EventHandler, EventSender, EventTarget, Receiver, Sender,
    },
    result::{RHomeError, RHomeResult},
    telnet::TelnetDriver,
    time::driver::TimeDriver,
    Manager, ManagerPtr,
};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};
use termion::{color, style};
use tokio::task;
use yaml_rust::{Yaml, YamlLoader};

type DeviceList = Arc<Mutex<HashMap<String, DeviceInfo>>>;
type DriverList = Arc<Mutex<HashMap<String, DriverPtr>>>;

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

pub(crate) fn manager() -> ManagerPtr {
    ManagerPtr::new(Box::new(ManagerImpl::new()))
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
                match ev.target {
                    EventTarget::Everyone => {
                        if *dev_name == ev.source {
                            continue;
                        }
                    }
                    EventTarget::EveryoneIncludeSender => {}
                    EventTarget::SenderOnly => {
                        if *dev_name != ev.source {
                            continue;
                        }
                    }
                    EventTarget::ManagerOnly => {
                        continue;
                    }
                }

                senders.push(dev.tx.clone());
            }
        }

        // Ensure that nothing is locked during sent
        for sender in senders {
            _ = sender.send(ev.clone());
        }
    }
}

impl EventSender for ManagerImpl {
    fn get_receiver(&self) -> Receiver {
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

    fn load_driver(&mut self, name: String, driver: DriverPtr) -> RHomeResult<()> {
        let mut drivers = self.drivers.lock().unwrap();
        drivers.insert(name, driver);
        Ok(())
    }

    fn add_device(&mut self, device: DevicePtr) -> RHomeResult<()> {
        let mut devices = self.devices.lock().unwrap();
        let (tx, rx) = channel();
        let name: String;
        {
            let dev = device.lock().unwrap();
            name = dev.name();
        }
        devices.insert(name.clone(), DeviceInfo { tx: tx });
        {
            let mut dev = device.lock().unwrap();
            let tx2 = self.tx.clone();
            dev.start(tx2)?;
        }
        task::spawn(async move {
            run_event_loop(rx, device).await;
        });

        println!(
            "{}{}{}{} : device '{}{}{}' added",
            color::Fg(color::Green),
            style::Bold,
            "manager",
            style::Reset,
            color::Fg(color::Cyan),
            name,
            style::Reset
        );
        Ok(())
    }
}

fn create_driver(
    name: &str,
    configuration: &Yaml,
    manager: &mut dyn Manager,
) -> RHomeResult<DriverPtr> {
    let mut d: Box<dyn Driver> = match name {
        "time" => Ok(TimeDriver::new()),
        "dummy" => Ok(DummyDriver::new()),
        "telnet" => Ok(TelnetDriver::new()),
        _ => Err(RHomeError::new(format!("unknown driver '{}'", name))),
    }?;
    d.load(configuration, manager)?;
    Ok(DriverPtr::new(d))
}
