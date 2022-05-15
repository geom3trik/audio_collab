


use vizia::prelude::*;

use crate::{AppEvent, AppData};

#[derive(Lens)]
pub struct ChatUI {
    current_message: String,
}

impl ChatUI {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            current_message: String::new(),
        }.build(cx, |cx|{
            Textbox::new(cx, ChatUI::current_message)
                .on_submit(|cx, text| cx.emit(AppEvent::SendMessage(text)))
                .width(Stretch(1.0));

            List::new(cx, AppData::messages, |cx, index, item|{
                Label::new(cx, item);
            });
        })
        .child_space(Pixels(20.0))
        .row_between(Pixels(20.0))
    }
}

impl View for ChatUI {

}