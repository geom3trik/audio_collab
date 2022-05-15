use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use vizia::prelude::*;

use crate::client_handler::ClientHandler;
use crate::server_handler::ServerHandler;
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

    pub server: ServerHandler,
    pub client: Option<ClientHandler>,
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
                self.server.start_server(cx);
            }

            AppEvent::Connect => {
                self.show_login = false;
                println!("Connect to server");
                self.client = Some(ClientHandler::new("localhost:7878".to_string()));
            }

            AppEvent::SendMessage(message) => {
                self.messages.push(message.clone());
                match self.client_or_host {
                    ClientOrHost::Client => self.client.as_mut().unwrap().send(message),
                    ClientOrHost::Host => self.server.send(message),
                }
                println!("Send message: {}", message);
            }

            AppEvent::AppendMessage(message) => {
                println!("Rcv message: {}", message);
                self.messages.push(message.clone());
            }
        });
    }
}
