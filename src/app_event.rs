use vizia::prelude::*;

use crate::{ClientOrHost, net::{UserMsg, UserMetadata}};

pub enum AppEvent {
    ToggleLoginScreen,

    SetClientOrHost(ClientOrHost),

    SetHostIP(String),
    SetHostPort(String),

    SetClientUsername(String),
    SetServerPassword(String),

    // Host starts the server
    StartServer,

    // Client connects to the server
    Connect,

    //
    SendMessage(String),

    // Append a received message to the list of messages
    AppendMessage(UserMsg),

    OpenColorPicker,
    CloseColorPicker,
    ChooseColor(Color),

    UpdateUsersMetadata(Vec<UserMetadata>),
}
