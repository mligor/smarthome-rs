use event::Event;
use manager::DeviceManager;

use crate::device::DeviceData;

extern crate async_channel;
extern crate tokio;
extern crate uuid;

mod device;
mod event;
mod manager;

#[tokio::main]
async fn main() {
    let (sender, receiver) = async_channel::unbounded::<Event>();

    {
        let mut manager = DeviceManager::new(&sender, &receiver);
        {
            let dev01 = DeviceData::new("device01".to_string(), &sender, &receiver);
            let dev02 = DeviceData::new("device02".to_string(), &sender, &receiver);

            // Add devices
            manager.add(dev01);
            manager.add(dev02);
        }
        println!("Starting manager");
        manager.run().await;
    }
}
