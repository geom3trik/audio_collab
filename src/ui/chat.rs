use vizia::prelude::*;

use crate::{AppData, AppEvent, UserMsg};

#[derive(Lens)]
pub struct ChatUI {
    current_message: String,
}

impl ChatUI {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            current_message: String::new(),
        }
        .build(cx, |cx| {
            // Message box
            Textbox::new(cx, ChatUI::current_message)
                .on_submit(|cx, text| cx.emit(AppEvent::SendMessage(text)))
                .width(Stretch(1.0));

            // List of messages
            List::new(cx, AppData::messages, |cx, _, item| {
                HStack::new(cx, |cx|{
                    Label::new(cx, item.then(UserMsg::username))
                        .width(Pixels(100.0));
                    Label::new(cx, item.then(UserMsg::message));
                });
            });
        })
        .child_space(Pixels(20.0))
        .row_between(Pixels(20.0))
    }
}

impl View for ChatUI {}
