use std::net::TcpStream;
use std::io::Error;

pub fn client(ip: String) {
    let mut _client = match create_client(ip) {
        Ok(client) => {
            println!("Successfully connected to IP: {}", client.peer_addr().unwrap());
            client
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
}

fn create_client(ip: String) -> Result<TcpStream, Error> {
    TcpStream::connect(ip)
}