use std::{
    net::{SocketAddr, TcpStream},
    sync::{mpsc::Sender, Arc, Mutex},
};

use vizia::context::ContextProxy;

use crate::{read_from_mut_stream, server_handler::User, AppEvent, Msg};

pub struct ServerThread {
    pub stream: Arc<Mutex<TcpStream>>,
    pub user: Arc<Mutex<User>>,
}

impl ServerThread {
    pub fn spawn(
        cx: &mut ContextProxy,
        stream: TcpStream,
        user: Arc<Mutex<User>>,
        tx: Sender<(SocketAddr, Msg)>,
    ) -> ServerThread {
        let stream_ref = Arc::new(Mutex::new(stream));
        let user_ref = user.clone();

        let stream_ref_2 = stream_ref.clone();

        cx.spawn(move |cx| {
            loop {
                match read_from_mut_stream(stream_ref.clone()) {
                    Ok(msg) => {
                        let usr = user_ref.lock().unwrap();

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
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        ServerThread {
            stream: stream_ref_2,
            user,
        }
    }
}
