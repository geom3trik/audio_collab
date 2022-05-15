use std::sync::{mpsc::Receiver, Arc, Mutex};

use vizia::prelude::*;

pub mod ui;
pub use ui::*;

pub mod app_data;
pub use app_data::*;

pub mod app_event;
pub use app_event::*;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("src/ui/connect_style.css")
            .expect("Failed to find stylesheet");

        AppData {
            client_or_host: ClientOrHost::Client,
            show_login: true,
            host_ip: String::new(),
            host_port: String::new(),
            client_username: String::new(),
            server_password: String::new(),
            messages: Vec::new(),
            client_stream: None,
            senders: Arc::new(Mutex::new(Vec::new())),
            receiver: None,
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
    .always_on_top(true)
    .run();
}
