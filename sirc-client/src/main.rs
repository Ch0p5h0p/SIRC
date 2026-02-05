mod crypto;

use std::net::{TcpListener, TcpStream};
use std::env;
use std::io;
use std::io::Write;
use x25519_dalek::{EphemeralSecret, PublicKey};

fn auth(stream: &mut TcpStream, private: &EphemeralSecret, public: &PublicKey) -> std::io::Result<()>{
    stream.write(b"HELLO");
    // TODO: handshake logic
    return Ok(());
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args[1] == "help" || args.len()<2 {
        println!("Usage:\n\tsirc-client connect <address> <port>");
        return Ok(());
    } else if args[1] == "connect" {
        if args.len() < 4 {
            println!("Usage: sirc-client connect <address> <port>");
            return Ok(());
        }
        let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", &args[2], &args[3]))?;
        let private = EphemeralSecret::random();
        let public = PublicKey::from(&private);

        auth(&mut stream, &private, &public)?;
        return Ok(());
    }
    return Ok(());
}
