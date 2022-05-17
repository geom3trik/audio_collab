use vizia::prelude::*;

use crate::client_handler::ClientHandler;
use crate::server_handler::ServerHandler;
use crate::{AppEvent, UserMsg};

#[derive(Lens)]
pub struct AppData {
    // Whether the login screen should be shown
    pub show_login: bool,

    pub show_color_picker: bool,

    pub client_or_host: ClientOrHost,

    // The host IP address. Used by the client to connect to the host.
    pub host_ip: String,
    // The host port. Used by the client to connect to the host.
    pub host_port: String,

    pub client_username: String,
    pub client_color: Color,

    pub server_password: String,

    // List of messages
    pub messages: Vec<UserMsg>,

    pub server: Option<ServerHandler>,
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
                self.server = Some(ServerHandler::new(cx, self.client_username.clone()));
            }

            AppEvent::Connect => {
                self.show_login = false;
                println!("Connect to server");
                let address = self.host_ip.clone() + ":" + &self.host_port;
                self.client = Some(ClientHandler::new(cx, address, self.client_username.clone()));
            }

            AppEvent::SendMessage(msg) => {

                let msg = UserMsg {
                    username: self.client_username.clone(),
                    message: msg.clone(),
                    color: self.client_color.to_string(),
                };

                self.messages.push(msg.clone());
                match self.client_or_host {
                    ClientOrHost::Client => self.client.as_mut().unwrap().send(&msg),
                    ClientOrHost::Host => self.server.as_mut().unwrap().send(&msg),
                    // ClientOrHost::Host => self.server.send(message),
                }
                println!("Send message: {:?}", msg);
            }

            AppEvent::AppendMessage(msg) => {
                println!("Rcv message: {:?}", msg);
                self.messages.push(msg.clone());
            }

            AppEvent::OpenColorPicker => {
                self.show_color_picker = true;
            }

            AppEvent::CloseColorPicker => {
                self.show_color_picker = false;
            }

            AppEvent::ChooseColor(color) => {
                self.client_color = color.clone();
                self.show_color_picker = false;
            }
        });
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum ClientOrHost {
    Client,
    Host,
}