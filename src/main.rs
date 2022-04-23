use std::time::Duration;

use device::Device;
use tokio::time::sleep;

mod device;
mod dummy;
mod event;
mod manager;
mod result;
mod time;
mod types;

#[tokio::main]
async fn main() {
    {
        let mut manager = manager::DeviceManager::new();

        // manager.add(
        //     "time".to_string(),
        //     Device::new(Box::new(time::TimeDevice::new())),
        // );

        {
            let mut mngr = manager.clone();
            tokio::spawn(async move {
                {
                    if let Err(err) = mngr.add_devices_from_config("config.yaml".to_string()) {
                        println!("unable to load configuration file: {:?}", err)
                    }
                };

                // let d = Device::new(Box::new(dummy::DummyDevice::new("light01".to_string())));
                // sleep(Duration::from_millis(1000)).await;
                // mngr.add("light1".to_string(), d.clone());
            });
        }

        //println!("Starting manager");
        manager.start().await;
    }
}
