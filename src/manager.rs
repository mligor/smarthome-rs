use crate::{
    device::Device,
    dummy::DummyDevice,
    event::{channel, Event, Sender},
    result::{Error, Result},
    time::TimeDevice,
};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};
use termion::{color, style};
use yaml_rust::YamlLoader;

pub struct DeviceManagerData {
    tx: Sender,
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
            value: Arc::new(Mutex::new(DeviceManagerData { tx })),
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&mut self, name: String, device: Device) {
        let n = name.clone();
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
        // if mngr.started
        {
            let tx = mngr.tx.clone();
            let tx2 = tx.clone();
            dev_copy.clone().start(tx);

            let ev = Event::new("start".to_string(), n);
            //let tx_for_thread = tx.clone();
            _ = tx2.send(ev);
        };
    }

    pub(crate) async fn start(&mut self) {
        let tx: Sender;
        {
            let mngr = self.value.lock().unwrap();
            tx = mngr.tx.clone();
        }

        println!("Starting manager message loop");
        let mut rx = tx.subscribe();

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
    }
}

fn create_device_with_type(device_type: &str) -> Result<Device> {
    match device_type {
        "time" => Ok(Device::new(Box::new(TimeDevice::new()))),
        "dummy" => Ok(Device::new(Box::new(DummyDevice::new()))),
        _ => Err(Error::new(format!("unknown device type '{}'", device_type))),
    }
}
