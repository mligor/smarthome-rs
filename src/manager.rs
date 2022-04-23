use crate::{
    device::Device,
    dummy::DummyDevice,
    event::{channel, Event, Receiver, Sender},
    result::{Error, Result},
    time::TimeDevice,
};
use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};
use termion::{color, style};
use yaml_rust::{Yaml, YamlLoader};

pub struct DeviceManagerData {
    tx: Sender,
}

type DeviceManagerValue = Arc<Mutex<DeviceManagerData>>;
type DeviceList = Arc<Mutex<HashMap<String, DeviceInfo>>>;

struct DeviceInfo {
    //device: Device,
    tx: Sender,
}

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

    pub fn add(&mut self, name: String, device: Device, configuration: &Yaml) {
        let n = name.clone();
        let dev_copy = device.clone();
        {
            let mut device = device.clone();
            device.set_name(name.clone());
            match device.configure(configuration) {
                Ok(_) => (),
                Err(err) => println!("error configuring device {}: {}", &name, err),
            }
        }
        let (t1, r1) = channel();
        let mngr = self.value.lock().unwrap();
        {
            let mut devices = self.devices.lock().unwrap();
            devices.insert(
                name,
                DeviceInfo {
                    // device: device,
                    tx: t1,
                },
            );
        }
        // if mngr.started
        {
            let tx = mngr.tx.clone();
            let tx2 = tx.clone();
            dev_copy.clone().start(tx, r1);

            let ev = Event::new("start".to_string(), n);
            //let tx_for_thread = tx.clone();
            _ = tx2.send(ev);
        };
    }

    pub(crate) async fn start(&mut self) {
        //let tx: Sender;
        let mut rx: Receiver;
        {
            let mngr = self.value.lock().unwrap();
            rx = mngr.tx.subscribe();
            //tx = mngr.tx.clone();
        }

        //println!("Starting manager message loop");
        //        let mut rx = tx.subscribe();
        loop {
            match rx.recv().await {
                Ok(ev) => {
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
                    self.add(device_name.to_string(), dev, device);
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
}

fn create_device_with_type(device_type: &str) -> Result<Device> {
    match device_type {
        "time" => Ok(Device::new(Box::new(TimeDevice::new()))),
        "dummy" => Ok(Device::new(Box::new(DummyDevice::new()))),
        _ => Err(Error::new(format!("unknown device type '{}'", device_type))),
    }
}
