use std::{
    collections::HashMap,
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

pub use vizia::prelude::*;

use crate::{
    read_from_stream, server_thread::ServerThread, MessageTrait, Msg, UserMetadata, UserMsg,
};

pub type Users = Arc<Mutex<HashMap<SocketAddr, (Arc<Mutex<User>>, ServerThread)>>>;

pub struct User {
    pub addr: SocketAddr,
    pub metadata: UserMetadata,
    pub client: TcpStream,
}

pub struct ServerHandler {
    pub server: TcpListener,
    pub users: Users,
    pub sender: Arc<Mutex<Sender<Msg>>>,

    pub stx: Arc<Mutex<Sender<Msg>>>,
    pub srx: Arc<Mutex<Receiver<Msg>>>,
}

impl ServerHandler {
    pub fn new() -> ServerHandler {
        let server = TcpListener::bind("127.0.0.1:7878").expect("Failed to start server");
        server.set_nonblocking(true).unwrap();

        let (stx, srx) = mpsc::channel::<Msg>();

        ServerHandler {
            server,
            users: Arc::new(Mutex::new(HashMap::new())),
            sender: Arc::new(Mutex::new(stx.clone())),

            stx: Arc::new(Mutex::new(stx)),
            srx: Arc::new(Mutex::new(srx)),
        }
    }

    pub fn start(&self, cx: &mut Context) {
        let server = self.server.try_clone().unwrap();
        let users = self.users.clone();
        let srx = self.srx.clone();

        cx.spawn(move |cx| {
            //let mut clients = vec![];
            let (tx, rx) = mpsc::channel::<(SocketAddr, Msg)>();

            loop {
                // New client connected
                if let Ok((mut socket, addr)) = server.accept() {
                    println!("Client {} connected", addr);

                    // Await for it's metadata
                    let metadata = read_from_stream(&mut socket).expect("Something went wrong");
                    if let Msg::Metadata(meta) = metadata {
                        let user = Arc::new(Mutex::new(User {
                            addr: addr.clone(),
                            metadata: meta.clone(),
                            client: socket.try_clone().expect("Uh, fuck you I guess"),
                        }));

                        // Add the new user to the DB
                        users.lock().unwrap().insert(
                            addr,
                            (
                                user.clone(),
                                // While spawning it's client thread
                                ServerThread::spawn(cx, socket, user.clone(), tx.clone()),
                            ),
                        );
                    }
                }

                // Relay messages from other clients
                if let Ok((msg_addr, msg)) = rx.try_recv() {
                    Self::relay_msg(msg_addr, msg, users.clone());
                }

                // Send messages from server to clients
                if let Ok(msg) = srx.lock().unwrap().try_recv() {
                    Self::direct_msg(msg, users.clone());
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }

    /// Relay a message coming from a client to all others
    pub fn relay_msg(msg_addr: SocketAddr, msg: Msg, users: Users) {
        for (addr, user) in users.lock().unwrap().iter_mut() {
            if msg_addr != *addr {
                user.0
                    .lock()
                    .unwrap()
                    .client
                    .write_all(&msg.to_bytes())
                    .expect("Failed to write to buffer");
            }
        }
    }

    /// Send a message coming from the server to all clients
    pub fn direct_msg(msg: Msg, users: Users) {
        for (_, (user, _st)) in users.lock().unwrap().iter_mut() {
            let mut usr = user.lock().unwrap();
            println!(
                "From user sending to user {}: {:?}",
                usr.metadata.username, msg
            );
            usr.client
                .write_all(&msg.to_bytes())
                .expect("Failed to write to buffer");
        }
    }

    pub fn send(&mut self, msg: &UserMsg) {
        println!("Send message from server: {:?}", msg);
        self.sender
            .lock()
            .unwrap()
            .send(Msg::UserMsg(msg.clone()))
            .expect("Failed to send message");
    }
}
