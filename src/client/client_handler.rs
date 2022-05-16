use std::{
    io::{Read, Write, ErrorKind},
    net::TcpStream,
    sync::mpsc::{self, TryRecvError, Sender}
};

pub use vizia::prelude::*;

use crate::AppEvent;

pub struct ClientHandler {
    pub sender: Sender<String>,
}

impl ClientHandler {

    pub fn new(cx: &mut Context, addr: String) -> ClientHandler {

        let mut client = TcpStream::connect(addr).expect("Failed to connect");
        client.set_nonblocking(true).unwrap();

        let (tx, rx) = mpsc::channel::<String>();

        cx.spawn(move |cx|{
            loop {
                let mut buff = vec![0; 512];
                match client.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        println!("Received msg: {:?}", msg);

                        cx.emit(AppEvent::AppendMessage(msg)).expect("Failed to send message to app");
                    }

                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Connection lost to server");
                        break;
                    }
                }

                match rx.try_recv() {
                    Ok(msg) => {
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(512, 0);
                        client.write_all(&buff).expect("Failed to send message to server");
                    }

                    Err(TryRecvError::Empty) => (),

                    Err(TryRecvError::Disconnected) => break,
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        ClientHandler { 
            sender: tx.clone(),
        }


    }

    pub fn send(&mut self, msg: &str) {
        println!("Send message from client: {}", msg);
        let mut buff = msg.to_string().into_bytes();
        buff.resize(512, 0);
        self.sender.send(msg.to_string()).expect("Failed to send message");
    }
}
