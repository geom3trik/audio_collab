use core::time;
use std::{
    collections::HashMap,
    io::{Read,Write, ErrorKind},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

pub use vizia::prelude::*;

use crate::AppEvent;

// pub struct ServerHandler {
//     pub users: Arc<Mutex<HashMap<SocketAddr, (Sender<String>, Receiver<String>)>>>,
//     pub started: bool,
//     pub address: String,
// }

// impl ServerHandler {
//     pub fn new(addr: String) -> ServerHandler {
//         ServerHandler {
//             users: Arc::new(Mutex::new(HashMap::new())),
//             started: false,
//             address: addr,
//         }
//     }

//     pub fn start_server(&mut self, cx: &mut Context) -> std::io::Result<()> {
//         self.started = true;
//         let addr = self.address.clone();
//         let users_ref = self.users.clone();
//         println!("Server started");
//         cx.spawn(move |cx| {
//             let receiver = TcpListener::bind(addr).expect("Failed");
//             for stream in receiver.incoming() {
//                 let stream = stream.expect("failed");
//                 let addr = stream.local_addr().unwrap();
//                 println!(
//                     "New client connected to server: IP&PORT: {}:{}",
//                     addr.ip(),
//                     addr.port()
//                 );

//                 let (svtx1, svrx1) = mpsc::channel();
//                 // TODO: Hook up the client sender end
//                 let (cltx2, clrx2) = mpsc::channel();

//                 users_ref.lock().unwrap().insert(addr, (svtx1, clrx2));

//                 cx.spawn(move |cx| {
//                     handle_message(cx, svrx1, stream).unwrap();
//                 });
//             }
//         });

//         Ok(())
//     }

//     pub fn send(&mut self, msg: &str) {
//         for (_user, (sender, _receiver)) in self.users.lock().unwrap().iter() {
//             sender
//                 .send(msg.to_string())
//                 .expect("Failed to send message from server to clients");
//         }
//     }
// }
// pub fn handle_message(
//     cx: &mut ContextProxy,
//     rx: Receiver<String>,
//     mut stream: TcpStream,
// ) -> std::io::Result<()> {
//     // Handle multiple access stream
//     let mut buf = [0; 512];
//     for _ in 0..1000 {
//         // let the receiver get a message from a sender
//         let bytes_read = stream.read(&mut buf)?;
//         println!("Message read from client. Bytes read: {}", bytes_read);
//         // sender stream in a mutable variable
//         if bytes_read == 0 {
//             return Ok(());
//         }

//         // for message in &rx {
//         //     println!("Send: {}", message);
//         // }

//         if let Ok(msg) = rx.try_recv() {
//             println!("Send from server: {}", msg);
//             stream.write(msg.as_bytes()).expect("Failed to send message from server to client");
//         } 

//         //stream.write(&buf[..bytes_read])?;
//         // Print acceptance message
//         //read, print the message sent
//         let message = String::from_utf8_lossy(&buf);
//         println!("from the sender:{}", message);
//         cx.emit(AppEvent::AppendMessage(message.to_string()))
//             .unwrap();
//         // And you can sleep this connection with the connected sender
//         thread::sleep(time::Duration::from_secs(1));
//     }
//     // success value
//     Ok(())
// }

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
            let (tx, rx) = mpsc::channel::<String>();

            loop {
                if let Ok((mut socket, addr)) = server.accept() {
                    println!("Client {} connected", addr);
                    clients.push(socket.try_clone().expect("Failed to clone client"));
                    
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
                                    cx.emit(AppEvent::AppendMessage(msg.clone()));
                                    tx.send(msg).expect("Failed to send message to rx");
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
                if let Ok(msg) = rx.try_recv() {
                    clients = clients.into_iter().filter_map(|mut client|{
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(512, 0);
                        client.write_all(&buff).map(|_| client).ok()
                    }).collect::<Vec<_>>();
                }

                // Send messages from server to clients
                if let Ok(msg) = srx.try_recv() {
                    clients = clients.into_iter().filter_map(|mut client|{
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(512, 0);
                        client.write_all(&buff).map(|_| client).ok()
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
