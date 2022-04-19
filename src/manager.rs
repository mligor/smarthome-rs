use crate::event::Sender;
use crate::event::{channel, Event};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::device::Device;

type DeviceList = HashMap<String, Device>;

pub struct DeviceManagerData {
    tx: Sender,
    devices: DeviceList,
    started: bool,
}

pub struct DeviceManager {
    value: Arc<Mutex<DeviceManagerData>>,
}

impl DeviceManager {
    pub fn new() -> DeviceManager {
        let (tx, rx) = channel();
        Self {
            value: Arc::new(Mutex::new(DeviceManagerData {
                tx,
                //rx,
                devices: HashMap::new(),
                started: false,
            })),
        }
    }

    pub fn add(&mut self, device: Device) {
        let dev_copy = device.clone();
        let mut mngr = self.value.lock().unwrap();
        mngr.devices.insert(device.name(), device);
        if mngr.started {
            let tx = mngr.tx.clone();
            dev_copy.clone().start(tx.clone());
        };
    }

    pub async fn start(&mut self) {
        let tx: Sender;
        // Start all devices first
        {
            //println!("Starting devices");
            let mut mngr = self.value.lock().unwrap();
            mngr.started = true;
            tx = mngr.tx.clone();
            for device in mngr.devices.values() {
                device.clone().start(tx.clone());
            }
        }

        //println!("Starting manager message loop");

        let mut rx = tx.subscribe();
        // tokio::spawn(async move {
        //     let init_event = Event::new("init".to_string(), None);
        //     _ = tx.send(init_event);
        // });

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

    fn handle_event(&self, ev: Event) {
        if let None = ev.source {
            return; // Ignore own events
        }
        println!("{}: {}", "manager", ev.name);
    }
}

impl Clone for DeviceManager {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
