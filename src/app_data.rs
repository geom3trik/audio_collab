use std::borrow::BorrowMut;
use std::io::{self, prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time;
use std::{str, thread};

use vizia::prelude::*;

use crate::{AppEvent, ClientOrHost};

#[derive(Lens)]
pub struct AppData {
    // Whether the login screen should be shown
    pub show_login: bool,

    pub client_or_host: ClientOrHost,

    // The host IP address. Used by the client to connect to the host.
    pub host_ip: String,
    // The host port. Used by the client to connect to the host.
    pub host_port: String,

    pub client_username: String,

    pub server_password: String,

    // List of messages
    pub messages: Vec<String>,

    pub client_stream: Option<TcpStream>,

    pub senders: Arc<Mutex<Vec<Sender<String>>>>,
    pub receiver: Option<Receiver<String>>,
}

impl AppData {
    fn connect_client(&mut self) -> std::io::Result<()> {
        self.client_stream = Some(TcpStream::connect("127.0.0.1:7878")?);

        Ok(())
    }

    fn start_server(&mut self, cx: &mut Context) -> std::io::Result<()> {
        let senders = self.senders.clone();
        cx.spawn(move |cx| {
            let receiver = TcpListener::bind("127.0.0.1:7878").expect("Failed");
            for stream in receiver.incoming() {
                let stream = stream.expect("failed");

                let (tx, rx) = mpsc::channel();
                senders.lock().unwrap().push(tx);

                cx.spawn(move |cx| {
                    handle_message(cx, rx, stream);
                });
            }
        });

        Ok(())
    }

    fn send_message(&mut self, message: &String) {
        match self.client_or_host {
            ClientOrHost::Client => {
                if let Some(stream) = &mut self.client_stream {
                    stream
                        .write(message.as_bytes())
                        .expect("Failed to send message");
                } else {
                    println!("No connected stream");
                }
            }

            ClientOrHost::Host => {
                for sender in self.senders.lock().unwrap().iter_mut() {
                    sender
                        .send(message.clone())
                        .expect("Failed to send message from server to clients");
                }
            }
        }
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetClientOrHost(client_or_host) => {
                self.client_or_host = *client_or_host;
            }

            AppEvent::ToggleLoginScreen => {
                self.show_login ^= true;
            }

            AppEvent::SetHostIP(ip) => {
                self.host_ip = ip.clone();
            }

            AppEvent::SetHostPort(port) => {
                self.host_port = port.clone();
            }

            AppEvent::SetClientUsername(username) => {
                self.client_username = username.clone();
            }

            AppEvent::SetServerPassword(password) => {
                self.server_password = password.clone();
            }

            AppEvent::StartServer => {
                self.show_login = false;
                println!("Start the server connection!");
                //cx.spawn(|cx|{
                self.start_server(cx).expect("Something went wrong");
                //});
            }

            AppEvent::Connect => {
                self.show_login = false;
                println!("Connect to server");
                // cx.spawn(|cx|{
                //     start_client();
                // });
                self.connect_client();
            }

            AppEvent::SendMessage(message) => {
                println!("Send message: {}", message);
                self.messages.push(message.clone());
                self.send_message(message);
            }

            AppEvent::AppendMessage(message) => {
                self.messages.push(message.clone());
            }
        });
    }
}

pub fn handle_message(
    cx: &mut ContextProxy,
    rx: Receiver<String>,
    mut stream: TcpStream,
) -> std::io::Result<()> {
    // Handle multiple access stream
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // let the receiver get a message from a sender
        let bytes_read = stream.read(&mut buf)?;
        // sender stream in a mutable variable
        if bytes_read == 0 {
            return Ok(());
        }

        for message in &rx {
            println!("Send: {}", message);
        }

        //stream.write(&buf[..bytes_read])?;
        // Print acceptance message
        //read, print the message sent
        let message = String::from_utf8_lossy(&buf);
        println!("from the sender:{}", message);
        cx.emit(AppEvent::AppendMessage(message.to_string()));
        // And you can sleep this connection with the connected sender
        thread::sleep(time::Duration::from_secs(1));
    }
    // success value
    Ok(())
}
