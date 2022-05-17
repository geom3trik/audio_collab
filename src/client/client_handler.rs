use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, Sender, TryRecvError},
};

pub use vizia::prelude::*;

use crate::{AppEvent, MessageTrait, UserMsg};

pub struct ClientHandler {
    pub username: String,
    pub sender: Sender<UserMsg>,
}

impl ClientHandler {
    pub fn new(cx: &mut Context, addr: String, username: String) -> ClientHandler {
        let mut client = TcpStream::connect(addr).expect("Failed to connect");
        client.set_nonblocking(true).unwrap();

        // Send metadata
        let mut buff = username.clone().into_bytes();
        buff.resize(512, 0);
        client
            .write_all(&buff)
            .expect("Failed to send message to server");

        let (tx, rx) = mpsc::channel::<UserMsg>();

        cx.spawn(move |cx| loop {
            let mut buff = vec![0; 512];
            match client.read(&mut buff) {
                Ok(length) => {
                    if length == 0 {
                        println!("Message is empty somehow");
                        continue;
                    }

                    let message = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let message = String::from_utf8(message).unwrap();
                    let msg = UserMsg::from_msg(&message);

                    println!("Received msg: {:?}", msg);

                    cx.emit(AppEvent::AppendMessage(msg))
                        .expect("Failed to send message to app");
                }

                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("Connection lost to server");
                    break;
                }
            }

            match rx.try_recv() {
                Ok(msg) => {
                    client
                        .write_all(&msg.to_bytes())
                        .expect("Failed to send message to server");
                }

                Err(TryRecvError::Empty) => (),

                Err(TryRecvError::Disconnected) => break,
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        });

        ClientHandler {
            username,
            sender: tx.clone(),
        }
    }

    pub fn send(&mut self, msg: &UserMsg) {
        println!("Send message from client: {:?}", msg);
        self.sender
            .send(msg.clone())
            .expect("Failed to send message");
    }
}
