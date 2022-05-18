use std::{
    net::TcpStream,
    sync::mpsc::{self, Sender, TryRecvError},
};

pub use vizia::prelude::*;

use crate::{read_from_stream, write_to_stream, AppEvent, Msg, UserCursor, UserMetadata, UserMsg};

pub struct ClientHandler {
    pub metadata: UserMetadata,
    pub sender: Sender<UserMsg>,
}

impl ClientHandler {
    pub fn new(cx: &mut Context, addr: String, metadata: UserMetadata) -> ClientHandler {
        let mut client = TcpStream::connect(addr).expect("Failed to connect");
        client.set_nonblocking(true).unwrap();

        // Send metadata
        write_to_stream(&mut client, &Msg::Metadata(metadata.clone()));

        let (tx, rx) = mpsc::channel::<UserMsg>();

        cx.spawn(move |cx| loop {
            // write_to_stream(
            //     &mut client,
            //     &Msg::UserCursor(UserCursor {
            //         user_metadata: metadata.clone(),
            //         cursor_position: cx.current,
            //     }),
            // );

            match read_from_stream(&mut client) {
                Ok(msg) => {
                    // Handle messages
                    match msg {
                        Msg::Metadata(_meta) => println!("Thanks, bud."),
                        Msg::UserMsg(usermsg) => {
                            cx.emit(AppEvent::AppendMessage(usermsg.clone()))
                                .expect("Failed to send message back to app");
                        }
                        Msg::UserCursor(cursormsg) => cx
                            .emit(AppEvent::ChangeCursorPosition(cursormsg.cursor_position))
                            .expect("Failed to send message back to app"),
                    }
                }
                Err(err) => match err {
                    crate::ReadStreamError::IOError(_err) => {
                        // eprintln!("IO Error while trying to read a new message")
                    }
                    crate::ReadStreamError::BuffSize0 => {
                        eprintln!("Next message buffer size was 0");
                        // TODO: Close connection
                        break;
                    }
                },
            }

            match rx.try_recv() {
                Ok(msg) => {
                    write_to_stream(&mut client, &Msg::UserMsg(msg));
                }

                Err(TryRecvError::Empty) => (),

                Err(TryRecvError::Disconnected) => break,
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        });

        ClientHandler {
            metadata,
            sender: tx.clone(),
        }
    }

    pub fn send(&mut self, msg: &UserMsg) {
        println!("Send message from client: {:?}", msg);
        self.sender
            .send(msg.clone())
            .expect("Failed to send message");
    }
}
