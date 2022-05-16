use std::{
    io::{Read,Write, ErrorKind},
    net::{SocketAddr, TcpListener},
    sync::{
        mpsc::{self, Sender},
    },
};

pub use vizia::prelude::*;

use crate::AppEvent;

pub struct ServerHandler {
    sender: Sender<String>,
}

impl ServerHandler {

    pub fn new(cx: &mut Context) -> ServerHandler {
        let server = TcpListener::bind("127.0.0.1:7878").expect("Failed to start server");
        server.set_nonblocking(true).unwrap();

        let (stx, srx) = mpsc::channel::<String>();

        cx.spawn(move |cx|{
            let mut clients = vec![];
            let (tx, rx) = mpsc::channel::<(SocketAddr, String)>();

            loop {
                if let Ok((mut socket, addr)) = server.accept() {
                    println!("Client {} connected", addr);
                    clients.push((addr, socket.try_clone().expect("Failed to clone client")));
                    
                    let tx = tx.clone();
                    cx.spawn(move |cx|{
                        loop {
                            let mut buff = [0; 512];
                            match socket.read_exact(&mut buff) {
                                Ok(_) => {
                                    println!("Received bytes: {}", buff.len());
                                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                                    let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                                    println!("{}, {:?}", addr, msg);
                                    cx.emit(AppEvent::AppendMessage(msg.clone())).expect("Failed to send message back to app");
                                    tx.send((addr, msg)).expect("Failed to send message to rx");
                                }

                                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                                Err(_) => {
                                    println!("Closing connection to: {}", addr);
                                    break;
                                }
                            }
                        }

                        std::thread::sleep(std::time::Duration::from_millis(100));
                    });
                
                }

                // Relay messages from other clients
                if let Ok((msg_addr, msg)) = rx.try_recv() {
                    clients = clients.into_iter().filter_map(|(addr, mut client)|{
                        if msg_addr != addr {
                            let mut buff = msg.clone().into_bytes();
                            buff.resize(512, 0);
                            client.write_all(&buff).expect("Failed to write to buffer");
                        }
                        Some((addr, client))
                    }).collect::<Vec<_>>();
                }

                // Send messages from server to clients
                if let Ok(msg) = srx.try_recv() {
                    clients = clients.into_iter().filter_map(|(addr, mut client)|{
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(512, 0);
                        client.write_all(&buff).expect("Failed to write to buffer");
                        Some((addr, client))
                    }).collect::<Vec<_>>();
                }



                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        ServerHandler { 
            sender: stx.clone(),
        }
    }

    pub fn send(&mut self, msg: &str) {
        println!("Send message from server: {}", msg);
        let mut buff = msg.to_string().into_bytes();
        buff.resize(512, 0);
        self.sender.send(msg.to_string()).expect("Failed to send message");
    }
}
