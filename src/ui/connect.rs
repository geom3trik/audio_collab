use std::net::Ipv4Addr;

use crate::{AppData, AppEvent, ClientOrHost};
use vizia::prelude::*;

pub struct ConnectUI {}

impl ConnectUI {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                // Tabs
                HStack::new(cx, |cx| {
                    Label::new(cx, "Client")
                        .cursor(CursorIcon::Hand)
                        .on_press(|cx| cx.emit(AppEvent::SetClientOrHost(ClientOrHost::Client)));
                    Label::new(cx, "Host")
                        .cursor(CursorIcon::Hand)
                        .on_press(|cx| cx.emit(AppEvent::SetClientOrHost(ClientOrHost::Host)));
                })
                .class("tabs");

                // Tab Indicator
                HStack::new(cx, |cx| {
                    Element::new(cx)
                        .class("indicator")
                        .checked(AppData::client_or_host.map(|c| *c == ClientOrHost::Host));
                })
                .height(Auto);

                // Content
                Binding::new(cx, AppData::client_or_host, |cx, c| {
                    if c.get(cx) == ClientOrHost::Client {
                        VStack::new(cx, |cx| {
                            HStack::new(cx, |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "IP Address:");
                                    InputBox::new(
                                        cx,
                                        AppData::host_ip,
                                        |cx, text| {
                                            if text.parse::<Ipv4Addr>().is_ok() {
                                                cx.emit(InputBoxEvent::Valid);
                                                cx.emit(AppEvent::SetHostIP(text));
                                            } else {
                                                cx.emit(InputBoxEvent::Invalid);
                                            }
                                        },
                                        "Invalid IPv4 Address!",
                                    );
                                })
                                .class("ip_input");

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Port:");
                                    InputBox::new(
                                        cx,
                                        AppData::host_port,
                                        |cx, text| {
                                            if text.parse::<u16>().is_ok() {
                                                cx.emit(InputBoxEvent::Valid);
                                                cx.emit(AppEvent::SetHostPort(text));
                                            } else {
                                                cx.emit(InputBoxEvent::Invalid);
                                            }
                                        },
                                        "Invalid port number!",
                                    );
                                })
                                .class("port_input");
                            })
                            .class("input_row");

                            HStack::new(cx, |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Username:");
                                    InputBox::new(
                                        cx,
                                        AppData::client_metadata.map(|m| m.username.clone()),
                                        |cx, text| {
                                            if !text.is_empty() {
                                                cx.emit(InputBoxEvent::Valid);
                                                cx.emit(AppEvent::SetClientUsername(text));
                                            } else {
                                                cx.emit(InputBoxEvent::Invalid);
                                            }
                                        },
                                        "Username cannot be empty!",
                                    );
                                })
                                .class("username_input");

                                color_picker(cx);

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Server Password:");
                                    Textbox::new(cx, AppData::server_password).on_submit(
                                        |cx, text| {
                                            cx.emit(AppEvent::SetServerPassword(text));
                                        },
                                    );
                                })
                                .class("password_input");
                            })
                            .class("input_row");

                            Button::new(
                                cx,
                                |cx| cx.emit(AppEvent::Connect),
                                |cx| Label::new(cx, "Connect"),
                            )
                            .cursor(CursorIcon::Hand)
                            .class("accent")
                            .class("start");
                        })
                        .class("content");
                    } else {
                        VStack::new(cx, |cx| {
                            HStack::new(cx, |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Username:");
                                    InputBox::new(
                                        cx,
                                        AppData::client_metadata.map(|m| m.username.clone()),
                                        |cx, text| {
                                            if !text.is_empty() {
                                                cx.emit(InputBoxEvent::Valid);
                                                cx.emit(AppEvent::SetClientUsername(text));
                                            } else {
                                                cx.emit(InputBoxEvent::Invalid);
                                            }
                                        },
                                        "Username cannot be empty!",
                                    );
                                })
                                .class("username_input");

                                color_picker(cx);
                            })
                            .class("input_row");

                            HStack::new(cx, |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Server Password:");
                                    Textbox::new(cx, AppData::server_password).on_submit(
                                        |cx, text| {
                                            cx.emit(AppEvent::SetServerPassword(text));
                                        },
                                    );
                                })
                                .class("password_input");
                            })
                            .class("input_row");

                            Button::new(
                                cx,
                                |cx| cx.emit(AppEvent::StartServer),
                                |cx| Label::new(cx, "Sart Server"),
                            )
                            .cursor(CursorIcon::Hand)
                            .class("accent")
                            .class("start");
                        })
                        .class("content");
                    }
                });
            });
            // .on_move(|cx| );
        })
    }
}

impl View for ConnectUI {
    fn element(&self) -> Option<&'static str> {
        Some("connect_view")
    }
}

fn color_picker(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Element::new(cx)
            .background_color(AppData::client_metadata.map(|m| Color::from(m.color.clone())))
            .class("picker")
            .cursor(CursorIcon::Hand)
            .on_press(|cx| cx.emit(AppEvent::OpenColorPicker));

        Popup::new(cx, AppData::show_color_picker, |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    color_circle(cx, Color::from("#F54E47"));
                    color_circle(cx, Color::from("#F5E447"));
                    color_circle(cx, Color::from("#47F558"));
                    color_circle(cx, Color::from("#4292ED"));
                })
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx| {
                    color_circle(cx, Color::from("#A242ED"));
                    color_circle(cx, Color::from("#ED42BD"));
                    color_circle(cx, Color::from("#F58853"));
                    color_circle(cx, Color::from("#653822"));
                })
                .col_between(Pixels(5.0));
            })
            .row_between(Pixels(5.0))
            .child_space(Pixels(5.0));
        })
        .on_blur(|cx| cx.emit(AppEvent::CloseColorPicker));
    })
    .class("color_picker")
    .child_space(Stretch(1.0))
    .size(Stretch(1.0))
    .width(Pixels(30.0));
}

fn color_circle(cx: &mut Context, color: Color) {
    Element::new(cx)
        .background_color(color)
        .border_radius(Percentage(50.0))
        .size(Pixels(20.0))
        .cursor(CursorIcon::Hand)
        .on_press(move |cx| cx.emit(AppEvent::ChooseColor(color)));
}

pub enum InputBoxEvent {
    Invalid,
    Valid,
}

#[derive(Lens)]
pub struct InputBox {
    is_invalid: bool,
}

impl InputBox {
    pub fn new<'a>(
        cx: &'a mut Context,
        lens: impl Lens<Target = String>,
        on_edit: impl Fn(&mut Context, String) + Send + Sync + 'static,
        invalid_text: &str,
    ) -> Handle<'a, Self> {
        Self { is_invalid: false }.build(cx, |cx| {
            Textbox::new(cx, lens)
                .on_edit(on_edit)
                .toggle_class("invalid", InputBox::is_invalid);
            Label::new(cx, invalid_text)
                .visibility(InputBox::is_invalid)
                .color(Color::red())
                .font_size(12.0)
                .position_type(PositionType::SelfDirected)
                .top(Percentage(100.0))
                .text_wrap(false);
        })
    }
}

impl View for InputBox {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|input_box_event, _| match input_box_event {
            InputBoxEvent::Valid => {
                self.is_invalid = false;
            }

            InputBoxEvent::Invalid => {
                self.is_invalid = true;
            }
        });
    }
}
