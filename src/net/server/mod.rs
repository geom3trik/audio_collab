pub mod server_handler;
pub mod server_thread;

pub const TCP_PORT: u64 = 7878;
pub const UDP_PORT: u64 = 7879;

pub const LISTENING_IP: &str = "127.0.0.1:7878";

pub const LOOP_AWAIT_MS: u64 = 20;
