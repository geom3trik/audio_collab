use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::runtime::Runtime;
use vizia::prelude::*;

use crate::client_handler::ClientHandler;
use crate::server_handler::ServerHandler;
use crate::{AppEvent, UserMetadata, UserMsg, TCP_LISTENING_IP};

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

    pub client_addr: Option<SocketAddr>,

    pub client_metadata: UserMetadata,
    pub clients: Vec<UserMetadata>,

    pub server_password: String,

    // List of messages
    pub messages: Vec<UserMsg>,

    pub server: Arc<Mutex<ServerHandler>>,
    pub client: Arc<Mutex<ClientHandler>>,
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
                self.client_metadata.username = username.clone();
            }

            AppEvent::SetServerPassword(password) => {
                self.server_password = password.clone();
            }

            AppEvent::StartServer => {
                self.show_login = false;
                println!("Start the server connection!");

                let server_ref = self.server.clone();

                cx.spawn(move |cx| {
                    let rt = Runtime::new().unwrap();

                    rt.block_on(async {
                        server_ref.lock().unwrap().start(TCP_LISTENING_IP, cx).await;
                    });
                });

                cx.emit(AppEvent::Connect);
            }

            AppEvent::Connect => {
                self.show_login = false;
                let address = format!("{}:{}", self.host_ip.clone(), self.host_port.clone());
                let meta = self.client_metadata.clone();
                let client = self.client.clone();

                cx.spawn(move |cx| {
                    let rt = Runtime::new().unwrap();

                    rt.block_on(async {
                        ClientHandler::connect(client, cx, address, meta).await;
                    });
                });
            }

            AppEvent::SendMessage(msg) => {
                let msg = UserMsg {
                    user_metadata: self.client_metadata.clone(),
                    message: msg.clone(),
                };

                self.messages.push(msg.clone());
                self.client.lock().unwrap().send(&msg);
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
