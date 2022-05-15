use std::str;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};


pub fn start_client() -> std::io::Result<()> {
    // connect
    // Struct used to start requests to the server.
    // Check TcpStream Connection to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    for _ in 0..1000 {
        // Allow sender to enter message input 
        let mut input = String::new();
        // First access the input message and read it
        io::stdin().read_line(&mut input).expect("Failed to read");
        // Write the message so that the receiver can access it 
        stream.write(input.as_bytes()).expect("failed to write");
        // Add buffering so that the receiver can read messages from the stream
        let mut reader =BufReader::new(&stream);
        // Check if this input message values are u8
        let mut buffer: Vec<u8> = Vec::new();
        // Read input information
        reader.read_until(b'\n',&mut buffer)?;
       
        println!("read from server:{}",str::from_utf8(&buffer).unwrap());
        println!("");
    }
    Ok(())
}