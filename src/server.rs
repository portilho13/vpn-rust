use std::{io, net::TcpListener};

pub fn server(ip: String) {
    let listener = match create_listener(ip) {
        Ok(t) => {
            println!("Started to listen on ip {}", t.local_addr().unwrap());
            t
        },
        Err(err) => {
            println!("Error: {}", err); 
            return
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client with IP: {} connected sucessfully", stream.peer_addr().unwrap());
            },
            Err (err) => {
                println!("Connection failed: {}", err)
            }
        }
    }
}

fn create_listener(ip: String) -> Result<TcpListener, io::Error> {
    let listener = TcpListener::bind(ip)?;
    Ok(listener)
}