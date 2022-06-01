use std::{sync::Arc, time::Duration};

use tokio::{net::UdpSocket, sync::Mutex};

use crate::net::{
    interchange::{assert_send_msg, read_frame_from_any},
    Msg, UserMetadata,
};

/// An abstraction that represents the channel from Client to Server
/// from the client's perspective.
pub struct ClientConnector {
    socket: Arc<UdpSocket>,
    send_msg_cache: Arc<Mutex<Vec<Msg>>>,
}

impl ClientConnector {
    /// Attempts to create a new ClientConnector while connecting to a server address
    pub async fn connect(
        addr: &str,
        metadata: Arc<std::sync::Mutex<UserMetadata>>,
    ) -> Result<Self, std::io::Error> {
        // We create the "connection" to the server
        let socket = UdpSocket::bind("localhost:7979").await?;
        socket.connect(addr).await.unwrap();
        let socket_ref = Arc::new(socket);
        let msg_cache = Arc::new(Mutex::new(Vec::new()));

        let connector = Self {
            socket: socket_ref.clone(),
            send_msg_cache: msg_cache.clone(),
        };

        let socket = socket_ref.clone();
        tokio::spawn(async move {
            loop {
                let mut msgs = msg_cache.lock().await;
                if !msgs.is_empty() {
                    // We take the next message in the buffer (we know there is one)
                    let next_msg = msgs.pop().unwrap();

                    // Wait for the socket to become writable
                    socket.writable().await.unwrap();
                    assert_send_msg(&*socket, &next_msg).await.unwrap();
                }

                std::thread::sleep(Duration::from_millis(200));
            }
        });

        tokio::spawn(async move {
            loop {
                match read_frame_from_any(&socket_ref).await {
                    Ok((frame, _addr)) => {
                        Self::handle_message(&frame.get_msg().unwrap()).await;
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

        // First thing we do: Send our metadata to the server we just connected
        // validating our connection.
        connector
            .send_msg(&Msg::Metadata(metadata.lock().unwrap().clone()))
            .await;

        Ok(connector)
    }

    async fn handle_message(msg: &Msg) {
        let _ = msg;
        todo!();
        // match msg {

        // }
    }

    pub async fn send_msg(&self, msg: &Msg) {
        self.send_msg_cache.lock().await.push(msg.clone());
    }
}
