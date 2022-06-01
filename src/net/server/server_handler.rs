use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use tokio::{
    net::UdpSocket,
    sync::{mpsc, Mutex},
};
pub use vizia::prelude::*;

use crate::{
    net::{
        interchange::{quick_send_msg, read_frame_from_any},
        server_thread::ServerThread,
        Msg, UserMetadata, LISTENING_IP, LOOP_AWAIT_MS,
    },
    AppEvent,
};

pub type Users = Arc<Mutex<HashMap<SocketAddr, (Arc<Mutex<User>>, ServerThread)>>>;

pub struct User {
    pub addr: SocketAddr,
    pub metadata: UserMetadata,
}

pub enum ServerHandlerStatus {
    Offline,
    Online,
    Panic,
}

pub struct ServerHandler {
    pub status: ServerHandlerStatus,
    pub socket: Arc<UdpSocket>, // The socket where users connect to
    pub users: Users,
}

impl ServerHandler {
    pub fn new(cx: &mut Context) -> ServerHandler {
        dbg!("Setting up the handler");
        let rt = tokio::runtime::Runtime::new().unwrap();
        dbg!("Runtime");
        let socket = rt.block_on(async {
            Arc::new(
                UdpSocket::bind(LISTENING_IP)
                    .await
                    .expect("Failed to start TCP server"),
            )
        });
        dbg!("Sockets");
        let socket_ref = socket.clone();

        let users = Arc::new(Mutex::new(HashMap::new()));
        let users_ref = users.clone();
        let users_ref2 = users.clone();

        cx.spawn(move |cx| {
            // Spawn a thread that creates each thread for each user
            let (tx, mut rx) = mpsc::channel::<(SocketAddr, Msg)>(16);
            rt.spawn(async move {
                loop {
                    let cx = cx.clone();
                    // Self::update_cursors(&mut cx, users.clone());
                    let tx_ref = tx.clone();

                    match read_frame_from_any(&socket).await {
                        Ok((frame, addr)) => {
                            if let Ok(msg) = frame.get_msg() {
                                if let Msg::Metadata(meta) = msg {
                                    // New client attempted to connect to us!
                                    let user = Arc::new(Mutex::new(User {
                                        addr,
                                        metadata: meta.clone(),
                                    }));
                                    users_ref.lock().await.insert(
                                        addr,
                                        (
                                            user.clone(),
                                            // While spawning it's client thread
                                            ServerThread::spawn(cx, addr, user.clone(), tx_ref)
                                                .await,
                                        ),
                                    );
                                }
                                // Self::handle_message(&frame.get_msg().unwrap()).await;
                            }
                        }

                        Err(ref e) => {
                            if e.kind() == tokio::io::ErrorKind::WouldBlock {
                                // False positive, no message could be read
                                continue;
                            }
                            panic!("{}", e)
                        }
                    }

                    std::thread::sleep(Duration::from_millis(200));
                }
            });

            rt.spawn(async move {
                loop {
                    // Relay messages from other clients
                    if let Ok((msg_addr, msg)) = rx.try_recv() {
                        Self::relay_msg(msg_addr, msg, users_ref2.clone()).await;
                    }

                    std::thread::sleep(std::time::Duration::from_millis(LOOP_AWAIT_MS));
                }
            });
        });

        ServerHandler {
            status: ServerHandlerStatus::Offline,
            socket: socket_ref,
            users,
        }
    }

    pub fn start(&mut self) {
        self.status = ServerHandlerStatus::Online;
    }

    /// Relay a message coming from a client to all others
    async fn relay_msg(msg_addr: SocketAddr, msg: Msg, users: Users) {
        for (addr, (_user, st)) in users.lock().await.iter_mut() {
            if msg_addr != *addr {
                quick_send_msg(&st.user_socket, &msg).await.unwrap();
            }
        }
    }

    async fn update_cursors(cx: ContextProxy, users: Users) {
        let mut users_meta = Vec::new();
        for (_addr, (user, _st)) in users.lock().await.iter() {
            users_meta.push(user.lock().await.metadata.clone());
        }

        cx.emit(AppEvent::UpdateUsersMetadata(users_meta))
            .expect("Failed to send message back to app");
    }
}
