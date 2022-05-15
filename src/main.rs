use vizia::prelude::*;

pub mod ui;
pub use ui::*;

pub mod app_data;
pub use app_data::*;

pub mod app_event;
pub use app_event::*;

pub mod server;
pub use server::*;

pub mod client;
pub use client::*;

fn main() {
    Application::new(|cx|{

        cx.add_stylesheet("src/ui/connect_style.css").expect("Failed to find stylesheet");

        AppData {
            show_login: true,
            host_ip: String::new(),
            host_port: String::new(),
            client_username: String::new(),
            server_password: String::new(),
            messages: Vec::new(),
            tcp_stream: None,
        }.build(cx);

        Binding::new(cx, AppData::show_login, |cx, show_login|{
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
