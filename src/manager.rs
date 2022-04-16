use crate::event::Event;
use async_channel::{Receiver, Sender};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task;

use crate::device::{run_device, Device};

type DeviceList = Arc<Mutex<HashMap<String, Device>>>;

pub struct DeviceManager {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    devices: DeviceList,
}

impl DeviceManager {
    pub fn new(sender: &Sender<Event>, receiver: &Receiver<Event>) -> DeviceManager {
        DeviceManager {
            sender: sender.clone(),
            receiver: receiver.clone(),
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn add(&mut self, device: Device) {
        let device_name;
        {
            let device_data = device.lock().unwrap();
            device_name = device_data.name.clone();
        }

        let mut devices = self.devices.lock().unwrap();
        devices.insert(device_name.clone(), device);
    }

    fn run_all_devices(devices: DeviceList) {
        let devices = devices.lock().unwrap();
        for device in devices.values() {
            run_device(device);
        }
    }

    pub async fn run(&self) {
        // run all devices
        DeviceManager::run_all_devices(self.devices.clone());
        task::spawn(async move { self.event_loop() });
    }

    async fn event_loop(&self) {
        loop {
            if let Ok(event) = self.receiver.recv().await {
                println!("Event {} happend.", event.name);
            } else {
                println!("Error");
            }
        }
    }
}
