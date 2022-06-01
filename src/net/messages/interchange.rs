use std::{net::SocketAddr, time::Duration};

pub const MSG_BUF_SIZE: usize = 512;

use tokio::{net::UdpSocket, time::timeout};

use crate::net::{
    udp_frame::{UdpFrame, UdpMsgType},
    Msg,
};

// UDP
pub const TIMEOUT_TRIES: usize = 2;
pub const TIMEOUT: Duration = Duration::from_millis(200);

/// Sends a message over the UdpSocket and doesn't check if the message arrived correctly
pub async fn quick_send_msg(socket: &UdpSocket, msg: &Msg) -> Result<(), std::io::Error> {
    match socket
        .send(&UdpFrame::from(msg, UdpMsgType::Response).to_bytes())
        .await
    {
        Ok(_len) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Sends a message over the UdpSocket and checks if the message arrived correctly by using timeouts
pub async fn assert_send_msg(socket: &UdpSocket, msg: &Msg) -> Result<(), std::io::Error> {
    // Try and send that message
    if let Err(e) = quick_send_msg(&*socket, &msg).await {
        return Err(e);
    }

    let mut buf = Vec::with_capacity(MSG_BUF_SIZE);
    let mut timeout_count = 0;
    while timeout_count < TIMEOUT_TRIES {
        // Try to receive a message with a timeout
        match timeout(TIMEOUT, socket.recv(&mut buf)).await {
            Err(_) => {
                dbg!("First timeout expired");
                timeout_count += 1;
                // Resend the message because of the timeout
                if let Err(e) = quick_send_msg(&*socket, &msg).await {
                    return Err(e);
                }
            }
            Ok(msg) => {
                if let Ok(_len) = msg {
                    dbg!("A message has been received to the buffer");
                    if UdpFrame::from_bytes(buf.as_slice()).is_request() {
                        dbg!("Got request.");
                        break;
                    } else {
                        dbg!("That message was weird indeed...");
                    }
                } else {
                    panic!("Socket receive error!");
                }
            }
        }
    }
    Ok(())
}

/// Awaits until the connected socket is readable and then tries to return a message from it.
pub async fn read_frame(socket: &UdpSocket) -> Result<UdpFrame, std::io::Error> {
    let mut buf = Vec::with_capacity(MSG_BUF_SIZE);
    // Wait for the socket to become readable
    socket.readable().await.unwrap();
    // Try and receive the next message
    match socket.try_recv(&mut buf) {
        Ok(_len) => Ok(UdpFrame::from_bytes(buf.as_slice())), // The message was read successfully
        Err(e) => Err(e),
    }
}

pub async fn read_frame_from_any(
    socket: &UdpSocket,
) -> Result<(UdpFrame, SocketAddr), std::io::Error> {
    let mut buf = Vec::with_capacity(512);
    // Wait for the socket to become readable
    socket.readable().await.unwrap();
    // Try and receive the next message
    match socket.recv_from(&mut buf).await {
        Ok((_len, addr)) => Ok((UdpFrame::from_bytes(buf.as_slice()), addr)),

        Err(e) => Err(e),
    }
}

// TCP STREAMS

// #[derive(Debug)]
// pub enum ReadStreamError {
//     IOError(std::io::Error),
//     BuffSize0,
// }

// impl From<std::io::Error> for ReadStreamError {
//     fn from(err: std::io::Error) -> Self {
//         ReadStreamError::IOError(err)
//     }
// }

// pub fn write_to_stream(stream: &mut TcpStream, msg: &Msg) {
//     let mut buff = msg.clone().to_bytes();
//     buff.resize(MSG_BUF_SIZE, 0);
//     stream
//         .write_all(&buff)
//         .expect("Failed to send message to server");
// }

// pub fn read_from_stream(stream: &mut TcpStream) -> Result<Msg, ReadStreamError> {
//     let mut buff = [0; MSG_BUF_SIZE];
//     let size = stream.read(&mut buff)?;

//     if size == 0 {
//         return Err(ReadStreamError::BuffSize0);
//     }

//     Ok(Msg::from_bytes(
//         &buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(),
//     ))
// }
