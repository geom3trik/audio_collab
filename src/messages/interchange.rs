use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream, UdpSocket},
};

pub const MSG_BUF_SIZE: usize = 512;

use crate::{MessageTrait, Msg};

// UDP

pub fn read_datagram(socket: &mut UdpSocket) -> (Msg, SocketAddr) {
    let mut buff = [0; MSG_BUF_SIZE];
    let (_read, addr) = socket.recv_from(&mut buff).unwrap();
    let message = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
    let msg = Msg::from_bytes(&message);
    (msg, addr)
}

pub fn write_datagram(socket: &mut UdpSocket, msg: &Msg) {
    let mut buff = msg.clone().to_bytes();
    buff.resize(MSG_BUF_SIZE, 0);
    let _sent = socket.send(&buff).unwrap();
}

// TCP STREAMS

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

pub fn write_to_stream(stream: &mut TcpStream, msg: &Msg) {
    let mut buff = msg.clone().to_bytes();
    buff.resize(MSG_BUF_SIZE, 0);
    stream
        .write_all(&buff)
        .expect("Failed to send message to server");
}

pub fn read_from_stream(stream: &mut TcpStream) -> Result<Msg, ReadStreamError> {
    let mut buff = [0; MSG_BUF_SIZE];
    let size = stream.read(&mut buff)?;

    if size == 0 {
        return Err(ReadStreamError::BuffSize0);
    }

    Ok(Msg::from_bytes(
        &buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(),
    ))
}
