use std::sync::{Arc, Mutex};

use crate::event::Event;
use async_channel::{Receiver, Sender};
use tokio::task;

pub type Device = Arc<Mutex<DeviceData>>;

pub struct DeviceData {
    pub name: String,
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    pub on: bool,
    //uuid: Uuid,
}

impl DeviceData {
    pub fn new(name: String, sender: &Sender<Event>, receiver: &Receiver<Event>) -> Device {
        Arc::new(Mutex::new(DeviceData {
            name,
            sender: sender.clone(),
            receiver: receiver.clone(),
            on: false,
        }))
    }

    // pub fn run(&self) {
    //     task::spawn(async move { self.runLoop().await });
    // }

    async fn run_loop(device: &Device) {
        let receiver = device.lock().unwrap().receiver.clone();
        loop {
            let event = receiver.recv().await.unwrap();
            task::spawn(async { handle_event(device, event) });
        }
    }

    // async fn runLoop() {
    //     loop {
    //         if let Ok(event) = self.receiver.recv().await {
    //             self.handleEvent(event)
    //         } else {
    //             println!("Error")
    //         }
    //     }
    // }
}

async fn handle_event(device: &Device, event: Event) {
    println!("Event in device {} happend.", event.name);
}

pub fn run_device(dev: &Device) {
    println!("Starting device.");
    let device = dev.lock().unwrap();
    println!("Starting device {}.", device.name);
}
