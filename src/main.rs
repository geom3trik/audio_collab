use std::sync::{Arc, Mutex};

use client::client_handler::ClientHandler;
use server::server_handler::ServerHandler;
use vizia::prelude::*;

pub mod ui;
pub use ui::*;

pub mod app_data;
pub use app_data::*;

pub mod app_event;
pub use app_event::*;

pub mod client;
pub use client::*;

pub mod server;
pub use server::*;

pub mod messages;
pub use messages::*;

static SUIT_SEMIBOLD: &[u8] = include_bytes!("resources/SUIT-SemiBold.ttf");

#[tokio::main]
async fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("src/ui/connect_style.css")
            .expect("Failed to find stylesheet");

        cx.add_font_mem("semi-bold", SUIT_SEMIBOLD);

        AppData {
            client_or_host: ClientOrHost::Client,
            show_login: true,
            show_color_picker: false,
            host_ip: String::from("127.0.0.1"),
            host_port: String::from("7878"),
            client_metadata: UserMetadata {
                username: String::from("Default"),
                color: String::from("F54E47"),
                cursor: (0.0, 0.0),
            },
            client_addr: None,
            clients: Vec::new(),
            server_password: String::new(),
            messages: Vec::new(),
            client: Arc::new(Mutex::new(ClientHandler::new())),
            server: Arc::new(Mutex::new(ServerHandler::new())),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Binding::new(cx, AppData::show_login, |cx, show_login| {
                if show_login.get(cx) {
                    ConnectUI::new(cx);
                } else {
                    ChatUI::new(cx);
                }
            });

            Binding::new(cx, AppData::client_metadata, |cx, client_metadata| {
                let client = client_metadata.get(cx);
                Binding::new(cx, AppData::clients, move |cx, clients| {
                    for user in clients.get(cx).iter() {
                        if client.username != user.username {
                            let dpi = cx.style().dpi_factor;
                            Element::new(cx)
                                .left(Pixels((user.cursor.0 / dpi as f32) - 8.0))
                                .top(Pixels((user.cursor.1 / dpi as f32) - 8.0))
                                .background_color(Color::from(user.color.clone()))
                                .border_radius(Percentage(50.0))
                                .size(Pixels(16.0))
                                .position_type(PositionType::SelfDirected);
                        }
                    }
                });
            });
        })
        .class("page");
    })
    //.always_on_top(true)
    .run();
}
