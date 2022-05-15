use crate::ClientOrHost;

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

    //
    AppendMessage(String),
}
