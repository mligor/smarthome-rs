use termion::{color, style};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    device::Device,
    event::{Event, EventHandler, EventTarget, Sender},
    result::RHomeResult,
};

pub struct TelnetDevice {
    name: String,
    listen_on: String,
}

impl TelnetDevice {
    pub fn new(name: String, listen_on: String) -> Self {
        Self { name, listen_on }
    }

    fn execute_command(&mut self, command: String) {
        println!(
            "console -> {}{}{}{}",
            color::Fg(color::Cyan),
            style::Bold,
            command.clone(),
            style::Reset
        );
    }
}

impl EventHandler for TelnetDevice {
    fn handle_event(&mut self, ev: Event) {
        if ev.name == "console_command" {
            if let Some(command) = ev.data.get("command") {
                self.execute_command(command.clone());
            }
        }
    }
}

impl Device for TelnetDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self, tx: Sender) -> RHomeResult<()> {
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
                                let mut event =
                                    Event::new("console_command".to_string(), my_name.clone());
                                event.data.insert("command", command.clone());
                                event.target = EventTarget::SenderOnly;
                                _ = tx.send(event);

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
        Ok(())
    }
}
