use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    time::Duration,
};

pub use vizia::prelude::*;

use crate::{
    interchange::{is_data_available, read_from_stream, write_to_stream, ReadStreamError},
    AppEvent, Msg, UserCursor, UserMetadata, UserMsg,
};

pub struct ClientHandler {
    pub sender: Sender<UserMsg>,
    rcv: Receiver<UserMsg>,
}

impl ClientHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<UserMsg>();
        Self {
            sender: tx,
            rcv: rx,
        }
    }

    pub async fn connect(
        handler: Arc<Mutex<Self>>,
        cx: ContextProxy,
        addr: String,
        metadata: UserMetadata,
    ) {
        println!("{}", addr);
        let mut client = TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(1))
            .expect("Failed to connect");
        client.set_nonblocking(true).unwrap();

        // Send metadata
        write_to_stream(&mut client, &Msg::Metadata(metadata.clone()));

        tokio::spawn(async move {
            loop {
                Self::send_cursor_info(cx.clone(), metadata.clone(), &mut client);

                if is_data_available(&mut client) {
                    match read_from_stream(&mut client) {
                        Ok(msg) => {
                            // Handle messages
                            if let Msg::UserMsg(usermsg) = msg {
                                cx.emit(AppEvent::AppendMessage(usermsg.clone()))
                                    .expect("Failed to send message back to app");
                            }
                        }
                        Err(err) => match err {
                            ReadStreamError::IOError(_err) => {
                                // eprintln!("IO Error while trying to read a new message {:?}", err)
                            }
                            ReadStreamError::BuffSize0 => {
                                eprintln!("Next message buffer size was 0");
                                // TODO: Close connection
                                break;
                            }
                        },
                    }
                }

                match handler.lock().unwrap().rcv.try_recv() {
                    Ok(msg) => {
                        write_to_stream(&mut client, &Msg::UserMsg(msg));
                    }

                    Err(TryRecvError::Empty) => (),

                    Err(TryRecvError::Disconnected) => break,
                }

                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        })
        .await
        .unwrap();
    }

    pub fn send(&mut self, msg: &UserMsg) {
        println!("Send message from client: {:?}", msg);
        self.sender
            .send(msg.clone())
            .expect("Failed to send message");
    }

    fn send_cursor_info(cx: ContextProxy, meta: UserMetadata, client: &mut std::net::TcpStream) {
        let mousex = *cx.cursorx.lock().unwrap();
        let mousey = *cx.cursory.lock().unwrap();
        write_to_stream(
            client,
            &Msg::UserCursor(UserCursor {
                user_metadata: meta,
                cursor: (mousex, mousey),
            }),
        );
    }
}
