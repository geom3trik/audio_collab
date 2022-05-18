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
                .on_submit(|cx, text| {
                    if !text.is_empty() {
                        cx.emit(AppEvent::SendMessage(text));
                        // cx.emit(TextEvent::StartEdit);
                    }
                })
                .width(Stretch(1.0));

            // List of messages
            List::new(cx, AppData::messages, |cx, _, item| {
                HStack::new(cx, |cx| {
                    avatar(cx, item);
                    Label::new(cx, item.then(UserMsg::message));
                })
                .col_between(Pixels(10.0))
                .height(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
            })
            .row_between(Pixels(10.0));
        })
        .child_space(Pixels(20.0))
        .row_between(Pixels(20.0))
    }
}

impl View for ChatUI {}

pub fn avatar<L: Lens<Target = UserMsg>>(cx: &mut Context, user: L) -> Handle<impl View> {
    Label::new(
        cx,
        user.clone()
            .then(UserMsg::user_metadata.map(|m| m.username.clone()))
            .map(|name| String::from(name.chars().nth(0).unwrap())),
    )
    .size(Pixels(32.0))
    .border_radius(Percentage(50.0))
    .background_color(user.then(UserMsg::user_metadata.map(|m| Color::from(m.color.clone()))))
    .child_space(Stretch(1.0))
    .font_size(24.0)
    .color(Color::white())
}
