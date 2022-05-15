use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver},
    thread,
};

pub use vizia::prelude::*;

use crate::AppEvent;

pub struct ClientHandler {
    pub tcp_stream: TcpStream,
}

impl ClientHandler {
    pub fn new(addr: String) -> ClientHandler {
        ClientHandler {
            tcp_stream: TcpStream::connect(addr).unwrap(),
        }
    }

    pub fn send(&mut self, msg: &str) {
        self.tcp_stream
            .write(msg.as_bytes())
            .expect("Failed to send message");
    }
}
