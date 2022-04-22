use std::time::Duration;

use device::Device;
use tokio::time::sleep;

mod device;
mod dummy;
mod event;
mod manager;
mod time;
mod types;

#[tokio::main]
async fn main() {
    {
        let mut manager = manager::DeviceManager::new();

        {
            //let dev01 = Device::new("device1".to_string());
            let time_dev = Device::new(Box::new(time::TimeDevice::new()));

            // Add devices
            //manager.add("dev1", dev01);
            manager.add("time".to_string(), time_dev);
        }

        {
            let mut mngr = manager.clone();
            tokio::spawn(async move {
                let d = Device::new(Box::new(dummy::DummyDevice::new("light01".to_string())));
                sleep(Duration::from_millis(1000)).await;
                mngr.add("light1".to_string(), d.clone());
            });
        }

        //println!("Starting manager");
        manager.start().await;
    }
}
