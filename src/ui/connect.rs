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
                        .on_press(|cx| cx.emit(AppEvent::SetClientOrHost(ClientOrHost::Client)));
                    Label::new(cx, "Host")
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
                                    Textbox::new(cx, AppData::host_ip).on_edit(|cx, text| {
                                        cx.emit(AppEvent::SetHostIP(text));
                                    });
                                })
                                .class("ip_input");

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Port:");
                                    Textbox::new(cx, AppData::host_port).on_edit(|cx, text| {
                                        cx.emit(AppEvent::SetHostPort(text));
                                    });
                                })
                                .class("port_input");
                            })
                            .class("input_row");

                            HStack::new(cx, |cx| {
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Username:");
                                    Textbox::new(cx, AppData::client_username).on_edit(|cx, text| {
                                        cx.emit(AppEvent::SetClientUsername(text));
                                    });
                                })
                                .class("username_input");

                                color_picker(cx);

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Server Password:");
                                    Textbox::new(cx, AppData::server_password).on_edit(|cx, text| {
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
                            .class("accent")
                            .class("start");
                        })
                        .class("content");
                    } else {
                        VStack::new(cx, |cx| {

                            HStack::new(cx, |cx|{
                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Username:");
                                    Textbox::new(cx, AppData::client_username).on_edit(|cx, text| {
                                        cx.emit(AppEvent::SetClientUsername(text));
                                    });
                                })
                                .class("username_input");
                                
                                color_picker(cx);
                            })
                            .class("input_row");


                            VStack::new(cx, |cx| {
                                Label::new(cx, "Server Password:");
                                Textbox::new(cx, AppData::server_password).on_edit(|cx, text| {
                                    cx.emit(AppEvent::SetServerPassword(text));
                                });
                            })
                            .class("password_input");

                            Button::new(
                                cx,
                                |cx| cx.emit(AppEvent::StartServer),
                                |cx| Label::new(cx, "Sart Server"),
                            )
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
            .border_radius(Percentage(50.0))
            .size(Pixels(30.0))
            .on_press(|cx| cx.emit(AppEvent::OpenColorPicker));

        Popup::new(cx, AppData::show_color_picker, |cx|{
            HStack::new(cx, |cx|{
                VStack::new(cx, |cx|{
                    
                    Element::new(cx)
                        .background_color(Color::from("#F54E47"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#F54E47"))));
                    Element::new(cx)
                        .background_color(Color::from("#F5E447"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#F5E447"))));
                    Element::new(cx)
                        .background_color(Color::from("#47F558"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#47F558"))));
                    Element::new(cx)
                        .background_color(Color::from("#4292ED"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#4292ED"))));
                    
                })
                .row_between(Pixels(5.0));

                VStack::new(cx, |cx|{
                    Element::new(cx)
                        .background_color(Color::from("#A242ED"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#A242ED"))));
                    Element::new(cx)
                        .background_color(Color::from("#ED42BD"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#ED42BD"))));
                    Element::new(cx)
                        .background_color(Color::from("#F58853"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#F58853"))));
                    Element::new(cx)
                        .background_color(Color::from("#653822"))
                        .border_radius(Percentage(50.0))
                        .size(Pixels(20.0))
                        .on_press(|cx| cx.emit(AppEvent::ChooseColor(Color::from("#653822"))));
                })
                .row_between(Pixels(5.0));
            })
            .col_between(Pixels(5.0))
            .child_space(Pixels(5.0));
        })
        .background_color(Color::white())
        .border_radius(Pixels(10.0))
        .top(Percentage(110.0))
        .size(Auto);
    })
    .child_space(Stretch(1.0))
    .size(Stretch(1.0))
    .width(Pixels(30.0));
}