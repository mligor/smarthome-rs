use std::{collections::HashMap, net::TcpStream, thread};

use esphome::Connection;

use crate::{
    device::Device,
    event::{Event, EventDataValue, EventHandler, Sender},
    result::RHomeResult,
};

pub struct EspHomeDevice {
    name: String,
    host: String,
    password: String,
}

impl EspHomeDevice {
    pub fn new(name: String, host: String, password: String) -> Self {
        Self {
            name,
            host,
            password,
        }
    }
}

impl EventHandler for EspHomeDevice {
    fn handle_event(&mut self, _ev: Event) {}
}

impl Device for EspHomeDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self, tx: Sender) -> RHomeResult<()> {
        let host = self.host.clone();
        let password = self.password.clone();
        let my_name = self.name.clone();

        tokio::spawn(async move {
            let mut stream = TcpStream::connect(host).unwrap();
            let mut write_stream = stream.try_clone().unwrap();

            let connection = Connection::new(&mut stream, &mut write_stream);
            let device = connection.connect().unwrap();
            println!("Connected to {}", device.server_info());

            let mut edefs = HashMap::new();

            if password != "" {
                match device.authenticate(&password) {
                    Ok(mut ad) => {
                        ad.device.ping().unwrap();
                        println!("Pong!");

                        println!("Device info={:?}", ad.device_info().unwrap());

                        let entities = ad.list_entities().unwrap();
                        for e in entities {
                            println!("entity {:?}", &e);
                            edefs.insert(e.key(), e);
                        }

                        let rx = ad.subscribe_states().unwrap();

                        // //loop
                        // {
                        //     ad.device.ping().unwrap();
                        //     std::thread::sleep(Duration::from_secs(1));

                        //     let entities = ad.list_entities().unwrap();
                        //     for e in entities {
                        //         let key = e.key();
                        //         let s = ad.device.connection.get_last_state_for_key(key).unwrap();

                        //         match e.kind() {
                        //             EntityKind::BinarySensor(ee) => {
                        //                 println!("- bin_sensor {}: {:?}", ee.object_id(), s)
                        //             }
                        //             EntityKind::Camera(ee) => {
                        //                 println!("- cam {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Climate(ee) => {
                        //                 println!("- clim {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Cover(ee) => {
                        //                 println!("- cover {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Fan(ee) => {
                        //                 println!("- fan {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Light(ee) => {
                        //                 println!("- light {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Number(ee) => {
                        //                 println!("- number {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Select(ee) => {
                        //                 println!("- select {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //             EntityKind::Sensor(ee) => {
                        //                 println!("- sensor {}: {:?}", ee.object_id(), s)
                        //             }
                        //             EntityKind::Services => {
                        //                 println!("- service {:?} ({:?}): {:?}", key, "services", s)
                        //             }
                        //             EntityKind::Switch(ee) => {
                        //                 println!("- switch {}: {:?}", ee.object_id(), s)
                        //             }
                        //             EntityKind::TextSensor(_ee) => {
                        //                 //println!("- text {:?} ({:?}): {:?}", key, ee, s)
                        //             }
                        //         }
                        //     }
                        // }

                        thread::spawn(move || loop {
                            match rx.recv() {
                                Ok(ev) => {
                                    let mut name = ev.key.to_string();

                                    if let Some(e) = edefs.get(&ev.key) {
                                        if let Some(ei) = e.extended_info() {
                                            name = ei.object_id();
                                        }
                                    }

                                    let mut event =
                                        Event::new("state".to_string(), my_name.clone());

                                    event.data.insert("entity", EventDataValue::String(name));

                                    let str_state = match ev.state {
                                        esphome::State::Binary(s) => EventDataValue::Bool(s),
                                        esphome::State::Measurement(s) => EventDataValue::Number(s),
                                        esphome::State::Text(s) => EventDataValue::String(s),
                                        esphome::State::LightState((s, _)) => {
                                            EventDataValue::Bool(s)
                                        }
                                        esphome::State::FanState(s) => EventDataValue::Bool(s),
                                        esphome::State::LockState(s) => {
                                            EventDataValue::String(format!("{:?}", s))
                                        }
                                    };

                                    event.data.insert("state", str_state);
                                    _ = tx.send(event);

                                    // println!(
                                    //     "- new_state for {}.{}: {:?}",
                                    //     my_name, name, ev.state
                                    // );
                                }
                                Err(err) => {
                                    println!("error receiving event in manager: {}", err)
                                }
                            }
                        });

                        ad.listen().unwrap();
                    }
                    Err(err) => println!("Auth Error {:?}", err),
                }

                // ...
            }
        });

        Ok(())
    }
}
