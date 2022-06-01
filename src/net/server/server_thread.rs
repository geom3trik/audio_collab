use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::UdpSocket,
    sync::{mpsc::Sender, Mutex},
};
use vizia::context::ContextProxy;

use crate::{net::{
    interchange::{assert_send_msg, read_frame},
    server_handler::User,
    Control, Msg,
}, AppEvent};

pub struct ServerThread {
    pub user_socket: Arc<UdpSocket>,
    pub user: Arc<Mutex<User>>,
}

impl ServerThread {
    pub async fn spawn(
        cx: ContextProxy,
        addr: SocketAddr,              // User address
        user: Arc<Mutex<User>>,        // The user from the database
        tx: Sender<(SocketAddr, Msg)>, // Client-thread to server-handler one-way connection
    ) -> ServerThread {
        let user_ref = user.clone();

        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap()); // Let the OS choose an available port for us
        socket.connect(addr).await.unwrap(); // Connect to the user
        let user_socket = socket.clone();

        // Redirect the client to use this socket instead
        socket.writable().await.unwrap();
        assert_send_msg(
            &socket,
            &Msg::Control(Control::ConnectionRedirect(socket.local_addr().unwrap())),
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            loop {
                // Wait for the socket to become readable
                socket.readable().await.unwrap();
                // Try and receive the next message
                match read_frame(&socket).await {
                    Ok(frame) => {
                        let mut usr = user.lock().await;
                        // Self::handle_message(&frame.get_msg().unwrap()).await;
                        let msg = frame.get_msg().unwrap();
                        match msg {
                            Msg::Metadata(_meta) => println!("Thanks, bud."),
                            Msg::UserMsg(usermsg) => {
                                println!("Message received from client: {:?}", usermsg.clone());
                                cx.emit(AppEvent::AppendMessage(usermsg.clone()))
                                    .expect("Failed to send message back to app");
                                tx.send((usr.addr, Msg::UserMsg(usermsg.clone())))
                                    .await
                                    .expect("Failed to send message to rx");
                            }
                            Msg::UserCursor(cursor) => {
                                println!("Cursor updated from client: {:?}", cursor);
                                usr.metadata.cursor = cursor.cursor;
                                tx.send((usr.addr, Msg::UserCursor(cursor.clone())))
                                    .await
                                    .expect("Failed to send message to rx");
                            }
                            _ => (),
                        }
                    }
                    Err(e) => {
                        if e.kind() == tokio::io::ErrorKind::WouldBlock {
                            // False positive, no message could be read
                            continue;
                        }
                        panic!("{}", e)
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        });

        ServerThread {
            user_socket,
            user: user_ref,
        }
    }
}
