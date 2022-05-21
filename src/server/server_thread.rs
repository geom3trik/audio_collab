use std::{
    io::ErrorKind,
    net::{SocketAddr, TcpStream},
    sync::{mpsc::Sender, Arc, Mutex},
};

use vizia::context::ContextProxy;

use crate::{
    interchange::{is_data_available, read_from_stream, ReadStreamError},
    server_handler::User,
    AppEvent, Msg,
};

pub struct ServerThread {
    pub stream: TcpStream,
    pub user: Arc<Mutex<User>>,
}

impl ServerThread {
    pub fn spawn(
        cx: ContextProxy,
        stream: TcpStream,
        user: Arc<Mutex<User>>,
        tx: Sender<(SocketAddr, Msg)>,
    ) -> ServerThread {
        let user_ref = user.clone();

        let mut stream_copy = stream.try_clone().unwrap();

        cx.spawn(move |cx| {
            loop {
                if is_data_available(&mut stream_copy) {
                    match read_from_stream(&mut stream_copy) {
                        Ok(msg) => {
                            let mut usr = user_ref.lock().unwrap();

                            // Handle messages
                            match msg {
                                Msg::Metadata(_meta) => println!("Thanks, bud."),
                                Msg::UserMsg(usermsg) => {
                                    println!("Message received from client: {:?}", usermsg.clone());
                                    cx.emit(AppEvent::AppendMessage(usermsg.clone()))
                                        .expect("Failed to send message back to app");
                                    tx.send((usr.addr, Msg::UserMsg(usermsg.clone())))
                                        .expect("Failed to send message to rx");
                                }
                                Msg::UserCursor(cursor) => {
                                    // println!("Cursor updated from client: {:?}", cursor);
                                    usr.metadata.cursor = cursor.cursor;
                                    tx.send((usr.addr, Msg::UserCursor(cursor.clone())))
                                        .expect("Failed to send message to rx");
                                }
                                _ => (),
                            }
                        }
                        Err(err) => match err {
                            ReadStreamError::IOError(ref err)
                                if err.kind() == ErrorKind::WouldBlock => {}

                            ReadStreamError::IOError(_err) => {
                                // eprintln!("IO Error while trying to read a new message")
                            }
                            ReadStreamError::BuffSize0 => {
                                eprintln!("Next message buffer size was 0");
                                // TODO: Close connection
                                // break;
                            }
                        },
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        });

        ServerThread { stream, user }
    }
}
