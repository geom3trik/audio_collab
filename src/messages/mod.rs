use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use vizia::prelude::*;

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
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UserMetadata {
    pub username: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UserMsg {
    pub user_metadata: UserMetadata,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Data, Lens)]
pub struct UserCursor {
    pub user_metadata: UserMetadata,
    pub cursor_position: (f32, f32),
}

impl MessageTrait<'_> for Msg {}

pub fn read_from_stream(stream: &mut TcpStream) -> Result<Msg, ReadStreamError> {
    let mut buff = [0; 512];
    let size = stream.read(&mut buff)?;

    if size == 0 {
        return Err(ReadStreamError::BuffSize0);
    }

    Ok(Msg::from_bytes(
        &buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(),
    ))
}

pub fn read_from_mut_stream(stream: Arc<Mutex<TcpStream>>) -> Result<Msg, ReadStreamError> {
    let mut buff = [0; 512];
    let size = stream.lock().unwrap().read(&mut buff)?;

    if size == 0 {
        return Err(ReadStreamError::BuffSize0);
    }

    Ok(Msg::from_bytes(
        &buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(),
    ))
}

pub fn write_to_stream(stream: &mut TcpStream, msg: &Msg) {
    let mut buff = msg.clone().to_bytes();
    buff.resize(512, 0);
    stream
        .write_all(&buff)
        .expect("Failed to send message to server");
}

#[derive(Debug)]
pub enum ReadStreamError {
    IOError(std::io::Error),
    BuffSize0,
}

impl From<std::io::Error> for ReadStreamError {
    fn from(err: std::io::Error) -> Self {
        ReadStreamError::IOError(err)
    }
}
