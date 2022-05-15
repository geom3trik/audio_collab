use std::str;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};

use vizia::prelude::*;

use crate::{AppEvent, start_server, start_client};

#[derive(Lens)]
pub struct AppData {

    // Whether the login screen should be shown
    pub show_login: bool,

    // The host IP address. Used by the client to connect to the host.
    pub host_ip: String,
    // The host port. Used by the client to connect to the host.
    pub host_port: String,

    pub client_username: String,

    pub server_password: String,

    // List of messages
    pub messages: Vec<String>,

    pub tcp_stream: Option<TcpStream>,
}

impl AppData {
    fn connect(&mut self) -> std::io::Result<()> {
        self.tcp_stream = Some(TcpStream::connect("127.0.0.1:7878")?);

        Ok(())
    }

    fn send_message(&mut self, message: &String) {
        if let Some(stream) = &mut self.tcp_stream {
            stream.write(message.as_bytes()).expect("Failed to send message");
        }
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {

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
                cx.spawn(|cx|{
                    start_server(cx).expect("Something went wrong");
                });
            }

            AppEvent::Connect => {
                self.show_login = false;
                println!("Connect to server");
                // cx.spawn(|cx|{
                //     start_client();
                // });
                self.connect();
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