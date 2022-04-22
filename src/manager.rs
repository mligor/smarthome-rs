use crate::{
    device::Device,
    event::{channel, Event, Sender},
    types::RHomeObject,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

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
            let init_event = Event::new("init".to_string(), id);
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

    fn handle_event(&self, ev: Event) {
        if self.id() == ev.source {
            return; // Ignore own events
        }
        println!("{}: Received event '{}'", "manager", ev.name);
    }
}

impl RHomeObject for DeviceManager {
    fn id(&self) -> Uuid {
        self.value.lock().unwrap().id
    }
}
