use vizia::prelude::*;
use crate::{AppData, AppEvent, ClientOrHost};

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
                                    Textbox::new(cx, AppData::host_ip).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetHostIP(text));
                                    });
                                })
                                .class("ip_input");

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Port:");
                                    Textbox::new(cx, AppData::host_port).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetHostPort(text));
                                    });
                                })
                                .class("port_input");
                            })
                            .class("input_row");

                            HStack::new(cx, |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Username:");
                                    Textbox::new(cx, AppData::client_username).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetClientUsername(text));
                                    });
                                })
                                .class("username_input");

                                color_picker(cx);

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Server Password:");
                                    Textbox::new(cx, AppData::server_password).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetServerPassword(text));
                                    });
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

                            HStack::new(cx, |cx|{
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Username:");
                                    Textbox::new(cx, AppData::client_username).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetClientUsername(text));
                                    });
                                })
                                .class("username_input");

                                color_picker(cx);
                            })
                            .class("input_row");

                            HStack::new(cx, |cx|{
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Server Password:");
                                    Textbox::new(cx, AppData::server_password).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetServerPassword(text));
                                    });
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
        })
    }
}

impl View for ConnectUI {
    fn element(&self) -> Option<&'static str> {
        Some("connect_view")
    }
}

fn color_picker(cx: &mut Context) {
    VStack::new(cx, |cx|{
        Element::new(cx)
            .background_color(AppData::client_color)
            .class("picker")
            .cursor(CursorIcon::Hand)
            .on_press(|cx| cx.emit(AppEvent::OpenColorPicker));

        Popup::new(cx, AppData::show_color_picker, |cx|{
            VStack::new(cx, |cx|{
                HStack::new(cx, |cx|{
                    color_circle(cx, Color::from("#F54E47"));
                    color_circle(cx, Color::from("#F5E447"));
                    color_circle(cx, Color::from("#47F558"));
                    color_circle(cx, Color::from("#4292ED"));
                })
                .col_between(Pixels(5.0));

                HStack::new(cx, |cx|{
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