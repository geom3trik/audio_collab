use std::sync::{Arc, Mutex};

use vizia::prelude::*;

use crate::{
    net::{
        client_handler::ClientHandler, server_handler::ServerHandler, Msg, UserMetadata, UserMsg,
    },
    AppEvent,
};

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

    pub client_metadata: UserMetadata,
    pub clients: Vec<UserMetadata>,

    pub server_password: String,

    // List of messages
    pub messages: Vec<UserMsg>,

    pub server: Option<ServerHandler>,
    pub client: ClientHandler,
}

impl Model for AppData {
    fn event(&mut self, mut cx: &mut Context, event: &mut Event) {
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
                self.client_metadata.username = username.clone();
            }

            AppEvent::SetServerPassword(password) => {
                self.server_password = password.clone();
            }

            AppEvent::StartServer => {
                self.show_login = false;
                println!("Start the server connection!");
                self.server = Some(ServerHandler::new(cx));

                self.server.as_mut().unwrap().start();

                let address = format!("{}:{}", self.host_ip.clone(), self.host_port.clone());

                self.client.connect_and_run(
                    &mut cx,
                    address,
                    Arc::new(Mutex::new(self.client_metadata.clone())),
                );
            }

            AppEvent::Connect => {
                self.show_login = false;
                println!("Connect to server");
                let address = format!("{}:{}", self.host_ip.clone(), self.host_port.clone());
                self.client.connect_and_run(
                    &mut cx,
                    address,
                    Arc::new(Mutex::new(self.client_metadata.clone())),
                );
            }

            AppEvent::SendMessage(msg) => {
                let msg = UserMsg {
                    user_metadata: self.client_metadata.clone(),
                    message: msg.clone(),
                };

                self.messages.push(msg.clone());
                self.client.send(&Msg::UserMsg(msg.clone()));
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
                self.client_metadata.color = color.to_string();
                self.show_color_picker = false;
            }

            AppEvent::UpdateUsersMetadata(v) => {
                self.clients = v.clone();
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum ClientOrHost {
    Client,
    Host,
}
