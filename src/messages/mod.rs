use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use vizia::prelude::*;

pub mod interchange;

pub trait MessageTrait<'a>: Sized + Deserialize<'a> + Serialize {
    fn from_msg(msg: &'a str) -> Self {
        ron::from_str(msg).unwrap()
    }

    fn to_msg(&'a self) -> String {
        ron::to_string(self).unwrap()
    }

    fn from_bytes(bytes: &'a [u8]) -> Self {
        // TODO: Fix the bytes problem
        // let _message = bytes
        //     .into_iter()
        //     .map(|x| *x)
        //     .take_while(|&x| x != 0)
        //     .collect::<Vec<_>>();
        ron::de::from_bytes(bytes).unwrap()
    }

    fn to_bytes(&'a self) -> Vec<u8> {
        self.to_msg().into_bytes()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub enum Msg {
    Metadata(UserMetadata),
    UserMsg(UserMsg),
    UserCursor(UserCursor),
    UsersCursors(UsersCursors),
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
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
    pub timestamp: i64,
    pub user_metadata: UserMetadata,
    pub cursors: Vec<(SocketAddr, f32, f32)>,
}

impl MessageTrait<'_> for Msg {}
