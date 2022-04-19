use std::time::Duration;

use manager::DeviceManager;
use tokio::time::sleep;

use crate::device::Device;

extern crate tokio;
extern crate uuid;

mod device;
mod event;
mod manager;

#[tokio::main]
async fn main() {
    {
        let mut manager = DeviceManager::new();
        {
            // let dev01 = Device::new("device1".to_string());
            // let dev02 = Device::new("device2".to_string());
            let dev03 = Device::new("dev3".to_string());

            // Add devices
            // manager.add(dev01).await;
            // manager.add(dev02).await;
            manager.add(dev03).await;
        }

        {
            let mut mngr = manager.clone();
            tokio::spawn(async move {
                let d = Device::new("light01".to_string());
                sleep(Duration::from_millis(1000)).await;
                mngr.add(d).await;
            });
        }

        {
            let mut mngr = manager.clone();
            tokio::spawn(async move {
                let d = Device::new("light02".to_string());
                sleep(Duration::from_millis(2000)).await;
                mngr.add(d).await;
            });
        }

        //println!("Starting manager");
        manager.start().await;
    }
}
