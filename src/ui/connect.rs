use vizia::prelude::*;

use crate::{AppData, AppEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum ClientOrHost {
    Client,
    Host,
}

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
                                    Textbox::new(cx, AppData::host_ip).on_submit(|cx, text| {
                                        cx.emit(AppEvent::SetClientUsername(text));
                                    });
                                })
                                .class("username_input");

                                VStack::new(cx, |cx| {
                                    Label::new(cx, "Server Password:");
                                    Textbox::new(cx, AppData::host_port).on_submit(|cx, text| {
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
                            VStack::new(cx, |cx| {
                                Label::new(cx, "Username:");
                                Textbox::new(cx, AppData::host_ip).on_submit(|cx, text| {
                                    cx.emit(AppEvent::SetClientUsername(text));
                                });
                            })
                            .class("username_input");

                            VStack::new(cx, |cx| {
                                Label::new(cx, "Server Password:");
                                Textbox::new(cx, AppData::host_port).on_submit(|cx, text| {
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
