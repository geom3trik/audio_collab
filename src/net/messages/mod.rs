use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use vizia::prelude::*;

pub mod interchange;
pub mod udp_frame;

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub enum Msg {
    Metadata(UserMetadata),
    UserMsg(UserMsg),
    UserCursor(UserCursor),
    UsersCursors(UsersCursors),
    Control(Control),
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens, PartialEq)]
pub struct UserMetadata {
    pub username: String,
    pub color: String,
    pub cursor: (f32, f32),
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UserMsg {
    pub user_metadata: UserMetadata,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UserCursor {
    pub user_metadata: UserMetadata,
    pub cursor: (f32, f32),
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UsersCursors {
    pub user_metadata: UserMetadata,
    pub cursors: Vec<(SocketAddr, f32, f32)>,
}

#[derive(Deserialize, Serialize, Debug, Data, Clone, PartialEq)]
pub enum Control {
    CloseConnection,
    ConnectionRedirect(SocketAddr),
}
