use termion::{color, style};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    device::Device,
    event::{Event, EventHandler},
};

#[derive(Default)]
pub struct ConsoleDevice {
    name: String,
    listen_on: String,
}

impl ConsoleDevice {
    pub fn new(name: String, listen_on: String) -> Self {
        ConsoleDevice { name, listen_on }
    }

    fn execute_command(&mut self, command: String) {
        println!(
            "{}{}{}{}",
            color::Fg(color::Cyan),
            style::Bold,
            command.clone(),
            style::Reset
        );
    }
}

impl EventHandler for ConsoleDevice {
    fn handle_event(&mut self, ev: Event) {
        println!(
            "{}{}{}{} : {}{}",
            color::Fg(color::Cyan),
            style::Bold,
            self.name,
            style::Reset,
            ev,
            style::Reset
        );
    }
}

impl Device for ConsoleDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self, tx: crate::event::Sender) -> bool {
        let listen_on = self.listen_on.clone();
        let my_name = self.name.clone();

        let prompt = format!(
            "{}{}{}{}> ",
            color::Fg(color::Green),
            style::Bold,
            self.name,
            style::Reset
        );

        tokio::spawn(async move {
            let listener = TcpListener::bind(listen_on).await.unwrap();
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let prompt = prompt.clone();
                let tx = tx.clone();
                let my_name = my_name.clone();

                tokio::spawn(async move {
                    let mut data = [0 as u8; 1024]; // using 1024 byte buffer
                    loop {
                        stream.write(prompt.as_bytes()).await.unwrap();
                        match stream.read(&mut data).await {
                            Ok(size) => {
                                let command = String::from_utf8(data[0..size].to_vec()).unwrap();
                                let command = command.trim().to_string();
                                tx.send(Event::new(command.clone(), my_name.clone()));
                                // echo everything!
                                // self.execute_command(
                                //     String::from_utf8(data[0..size].to_vec()).unwrap(),
                                // );
                                //stream.write(&data[0..size]).await.unwrap();
                                if command == "exit".to_string() {
                                    break;
                                }
                            }
                            Err(_) => {
                                println!(
                                    "An error occurred, terminating connection with {}",
                                    stream.peer_addr().unwrap()
                                );
                                stream.shutdown().await.unwrap();
                                break;
                            }
                        }
                    }
                });
            }
        });
        true
    }
}
