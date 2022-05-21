use std::{
    collections::HashMap,
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
};

pub use vizia::prelude::*;

use crate::{
    interchange::read_from_stream, server_thread::ServerThread, AppEvent, MessageTrait, Msg,
    UserMetadata, LOOP_AWAIT_MS, TCP_LISTENING_IP,
};

pub type Users = Arc<Mutex<HashMap<SocketAddr, (Arc<Mutex<User>>, ServerThread)>>>;

pub struct User {
    pub addr: SocketAddr,
    pub metadata: UserMetadata,
    pub client: TcpStream,
}

pub struct ServerHandler {
    pub users: Users,
}

impl ServerHandler {
    pub fn new() -> ServerHandler {
        ServerHandler {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start(&mut self, _addr: &str, cx: ContextProxy) {
        let tcp_server = TcpListener::bind(TCP_LISTENING_IP).expect("Failed to start TCP server");
        tcp_server.set_nonblocking(true).unwrap();

        let tcp_users = self.users.clone();

        println!("Server started");

        tokio::spawn(async move {
            //let mut clients = vec![];
            let (tx, rx) = mpsc::channel::<(SocketAddr, Msg)>();

            loop {
                Self::update_cursors(cx.clone(), tcp_users.clone());

                // New client connected
                if let Ok((mut socket, addr)) = tcp_server.accept() {
                    println!("Client {} connected", addr);

                    // Await for it's metadata
                    if let Msg::Metadata(meta) =
                        read_from_stream(&mut socket).expect("Something went wrong")
                    {
                        println!("Client connected successfuly");
                        let user = Arc::new(Mutex::new(User {
                            addr,
                            metadata: meta.clone(),
                            client: socket.try_clone().expect("Uh, fuck you I guess"),
                        }));

                        // Add the new user to the DB
                        tcp_users.lock().unwrap().insert(
                            addr,
                            (
                                user.clone(),
                                // While spawning it's client thread
                                ServerThread::spawn(cx.clone(), socket, user.clone(), tx.clone()),
                            ),
                        );
                    } else {
                        eprintln!("Something wrong happened here.");
                    }
                }

                // Relay messages from other clients
                if let Ok((msg_addr, msg)) = rx.try_recv() {
                    Self::relay_msg(msg_addr, msg, tcp_users.clone());
                }

                std::thread::sleep(std::time::Duration::from_millis(LOOP_AWAIT_MS));
            }
        })
        .await
        .unwrap();

        println!("Server closed");
    }

    /// Relay a message coming from a client to all others
    pub fn relay_msg(msg_addr: SocketAddr, msg: Msg, users: Users) {
        for (addr, (user, _st)) in users.lock().unwrap().iter_mut() {
            if msg_addr != *addr {
                user.lock()
                    .unwrap()
                    .client
                    .write_all(&msg.to_bytes())
                    .expect("Failed to write to buffer");
            }
        }
    }

    pub fn update_cursors(cx: ContextProxy, users: Users) {
        let users_metadata = users
            .lock()
            .unwrap()
            .iter()
            .map(|(_, (user, _))| user.lock().unwrap().metadata.clone())
            .collect::<Vec<_>>();

        cx.emit(AppEvent::UpdateUsersMetadata(users_metadata))
            .expect("Failed to send message back to app");
    }
}
