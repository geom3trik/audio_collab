use std::sync::{Arc, Mutex};

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

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("src/ui/connect_style.css")
            .expect("Failed to find stylesheet");

        AppData {
            client_or_host: ClientOrHost::Client,
            show_login: true,
            host_ip: String::from("127.0.0.1"),
            host_port: String::from("7878"),
            client_username: String::from("TODO"),
            server_password: String::from("TODO"),
            messages: Vec::new(),
            client: None,
            server: None,
        }
        .build(cx);

        Binding::new(cx, AppData::show_login, |cx, show_login| {
            if show_login.get(cx) {
                ConnectUI::new(cx);
            } else {
                ChatUI::new(cx);
            }
        });
    })
    // .always_on_top(true)
    .run();
}
