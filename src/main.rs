use vizia::{image::Pixels, prelude::*};

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

static SUIT_Semi_Bold: &[u8] = include_bytes!("resources/SUIT-SemiBold.ttf");

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("src/ui/connect_style.css")
            .expect("Failed to find stylesheet");

        cx.add_font_mem("semi-bold", SUIT_Semi_Bold);

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
            clients: Vec::new(),
            server_password: String::new(),
            messages: Vec::new(),
            client: None,
            server: None,
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

            Binding::new(cx, AppData::clients, |cx, clients| {
                for user in clients.get(cx).iter() {
                    let dpi = cx.style().dpi_factor;
                    Element::new(cx)
                        .left(Pixels((user.cursor.0 / dpi as f32) - 8.0))
                        .top(Pixels((user.cursor.1 / dpi as f32) - 8.0))
                        .background_color(Color::from(user.color.clone()))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(16.0))
                        .position_type(PositionType::SelfDirected);
                }
            });
        })
        .class("page");
    })
    //.always_on_top(true)
    .run();
}
