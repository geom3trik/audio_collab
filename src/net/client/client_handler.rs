use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

pub use vizia::prelude::*;

use crate::net::{client_connector::ClientConnector, Msg, UserCursor, UserMetadata};

pub struct ClientHandler {
    is_connected: Arc<Mutex<bool>>,
    tx: Option<Sender<Msg>>,
}

impl ClientHandler {
    pub fn new() -> Self {
        Self {
            is_connected: Arc::new(Mutex::new(false)),
            tx: None,
        }
    }

    pub fn connect_and_run(
        &mut self,
        cx: &mut Context,
        addr: String,
        metadata: Arc<Mutex<UserMetadata>>,
    ) {
        let is_connected = self.is_connected.clone();
        let (tx, rx) = channel();

        self.tx = Some(tx);

        cx.spawn(move |cx| {
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async {
                match ClientConnector::connect(&addr, metadata.clone()).await {
                    Ok(connector) => {
                        dbg!("Client connected: {}", addr);
                        *is_connected.lock().unwrap() = true;

                        let meta = metadata.clone();
                        loop {
                            // Send cursor info
                            let meta = meta.lock().unwrap().clone();
                            let mousex = *cx.cursorx.lock().unwrap();
                            let mousey = *cx.cursory.lock().unwrap();
                            connector
                                .send_msg(&Msg::UserCursor(UserCursor {
                                    user_metadata: meta,
                                    cursor: (mousex, mousey),
                                }))
                                .await;

                            if let Ok(msg) = rx.try_recv() {
                                connector.send_msg(&msg).await;
                            }

                            std::thread::sleep(std::time::Duration::from_millis(200));
                        }
                    }
                    Err(err) => {
                        dbg!("Connection refused: {}", err);
                    }
                }
            });
        });
    }

    pub fn send(&mut self, msg: &Msg) {
        if *self.is_connected.lock().unwrap() {
            dbg!("Sending msg");
            self.tx.as_ref().unwrap().send(msg.clone()).unwrap();
        }
    }
}
