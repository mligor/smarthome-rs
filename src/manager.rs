use crate::{
    device::{self, Device},
    dummy::DummyDevice,
    event::{channel, Event, Sender},
    result::{Error, Result},
    time::TimeDevice,
    types::RHomeObject,
};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};
use termion::{color, style};
use uuid::Uuid;
use yaml_rust::YamlLoader;

pub struct DeviceManagerData {
    id: Uuid,
    tx: Sender,
    started: bool,
}

type DeviceManagerValue = Arc<Mutex<DeviceManagerData>>;
type DeviceList = Arc<Mutex<HashMap<String, Device>>>;

#[derive(Clone)]
pub struct DeviceManager {
    value: DeviceManagerValue,
    devices: DeviceList,
}
impl DeviceManager {
    pub(crate) fn new() -> DeviceManager {
        let (tx, _) = channel();
        DeviceManager {
            value: Arc::new(Mutex::new(DeviceManagerData {
                id: Uuid::new_v4(),
                tx,
                started: false,
            })),
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&mut self, name: String, device: Device) {
        let dev_copy = device.clone();
        {
            let mut device = device.clone();
            device.set_name(name.clone());
        }
        let mngr = self.value.lock().unwrap();
        {
            let mut devices = self.devices.lock().unwrap();
            devices.insert(name, device);
        }
        if mngr.started {
            let tx = mngr.tx.clone();
            dev_copy.clone().start(tx.clone());
        };
    }

    pub(crate) async fn start(&mut self) {
        let tx: Sender;
        // Start all devices first
        {
            println!("Starting devices");
            let mut mngr = self.value.lock().unwrap();
            mngr.started = true;
            tx = mngr.tx.clone();
            let devices = self.devices.lock().unwrap();
            for device in devices.values() {
                device.clone().start(tx.clone());
            }
        }

        println!("Starting manager message loop");

        let mut rx = tx.subscribe();
        let id = self.id();
        tokio::spawn(async move {
            let init_event = Event::new("init".to_string(), id, "manager".to_string());
            _ = tx.send(init_event);
        });

        loop {
            match rx.recv().await {
                Ok(ev) => {
                    //                    self.handle_event(ev);
                    let mngr = self.clone();
                    tokio::spawn(async move {
                        mngr.handle_event(ev);
                    });
                }
                Err(err) => println!("error receiving event in manager: {}", err),
            }
        }
    }

    pub(crate) fn add_devices_from_config(&mut self, config_file: String) -> Result<()> {
        let content = fs::read_to_string(config_file)?;

        let docs = YamlLoader::load_from_str(&content)?;
        let doc = &docs[0]; // Take only first document (for now)
        let devices = &doc["devices"].as_hash().unwrap();

        for (device_name, device) in devices.iter() {
            let device_type = device["type"].as_str().unwrap();
            let device_name = device_name.as_str().unwrap();

            //println!("device = {:?} / {:?}", device_name, device_type);
            match create_device_with_type(device_type) {
                Ok(dev) => {
                    self.add(device_name.to_string(), dev);
                }
                Err(err) => {
                    println!(
                        "{}{}error: unable to create device '{:?}' : {:?}{}",
                        color::Fg(color::Red),
                        style::Bold,
                        device_name,
                        err.to_string(),
                        style::Reset
                    );
                    // eprintln!(
                    //     "unable to create device '{:?}' : {:?}",
                    //     device_name,
                    //     err.to_string(),
                    // );
                }
            }
        }
        Ok(())
    }

    fn handle_event(&self, ev: Event) {
        if self.id() == ev.source {
            return; // Ignore own events
        }

        println!(
            "{}{}{}{} : {}{}",
            color::Fg(color::Green),
            style::Bold,
            "manager",
            style::Reset,
            ev.name,
            style::Reset
        );
    }
}

fn create_device_with_type(device_type: &str) -> Result<Device> {
    match device_type {
        "time" => Ok(Device::new(Box::new(TimeDevice::new()))),
        "dummy" => Ok(Device::new(Box::new(DummyDevice::new()))),
        _ => Err(Error::new(format!("unknown device type '{}'", device_type))),
    }
}

impl RHomeObject for DeviceManager {
    fn id(&self) -> Uuid {
        self.value.lock().unwrap().id
    }
}
