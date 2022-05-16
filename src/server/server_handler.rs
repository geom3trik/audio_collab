use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
};

pub use vizia::prelude::*;

use crate::{AppEvent, UserMsg, MessageTrait};

struct User {
    pub name: String,
    pub client: TcpStream,
}

pub struct ServerHandler {
    pub sender: Sender<UserMsg>,
}

impl ServerHandler {
    pub fn new(cx: &mut Context, username: String) -> ServerHandler {
        let server = TcpListener::bind("127.0.0.1:7878").expect("Failed to start server");
        server.set_nonblocking(true).unwrap();

        let (stx, srx) = mpsc::channel::<UserMsg>();

        let users = Arc::new(Mutex::new(HashMap::new()));

        cx.spawn(move |cx| {
            //let mut clients = vec![];
            let (tx, rx) = mpsc::channel::<(SocketAddr, UserMsg)>();

            // Wait for clients to connect
            loop {
                if let Ok((mut socket, addr)) = server.accept() {
                    println!("Client {} connected", addr);

                    // Await for it's metadata
                    let mut buff = [0; 512];
                    socket.read_exact(&mut buff);
                    let msg = String::from_utf8(
                        buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(),
                    )
                    .unwrap();

                    // Add the new user to the DB
                    users.lock().unwrap().insert(
                        addr,
                        User {
                            name: msg,
                            client: socket.try_clone().expect("Uh, fuck you I guess"),
                        },
                    );

                    let tx = tx.clone();
                    let users_ref = users.clone();
                    cx.spawn(move |cx| {
                        loop {
                            let mut buff = [0; 512];
                            match socket.read(&mut buff) {
                                Ok(length) => {
                                    println!("Received bytes: {}", buff.len());

                                    if length == 0 {
                                        println!("Message couldn't be read correctly somehow. LENGTH = 0");
                                        continue;
                                    }


                                    let msg = UserMsg::from_bytes(&buff);
                                        
                                    if let Some(user) = users_ref.lock().unwrap().get(&addr) {
                                        println!("{}, {:?}", addr, msg);
                                        cx.emit(AppEvent::AppendMessage(msg.clone()))
                                            .expect("Failed to send message back to app");
                                        tx.send((addr, msg)).expect("Failed to send message to rx");
                                    }

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
                    for (addr, user) in users.lock().unwrap().iter_mut() {
                        if msg_addr != *addr {
                            user.client
                                .write_all(&msg.to_bytes())
                                .expect("Failed to write to buffer");
                        }
                    }
                }

                // Send messages from server to clients
                if let Ok(msg) = srx.try_recv() {
                    for (_, user) in users.lock().unwrap().iter_mut() {
                        user.client
                            .write_all(&msg.to_bytes())
                            .expect("Failed to write to buffer");
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        ServerHandler { 
            sender: stx.clone(),
        }
    }

    pub fn send(&mut self, msg: &UserMsg) {
        println!("Send message from server: {:?}", msg);
        self.sender
            .send(msg.clone())
            .expect("Failed to send message");
    }
}
