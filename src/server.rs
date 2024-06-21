use std::{io, net::TcpListener};
use ::tun::Device;
use std::sync::{Mutex, Arc};
use std::thread;
use crate::tun::{self, read_from_tun_and_send_to_client};

pub fn server(ip: String) {
    let listener = match create_listener(ip.clone()) {
        Ok(t) => {
            println!("Started to listen on IP {}", t.local_addr().unwrap());
            t
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let tun_iface = match tun::create_tun_iface() {
        Ok(t) => {
            println!("Iface {} created successfully", t.name().unwrap());
            t
        },
        Err(e) => {
            println!("Error creating Iface: {}", e);
            return;
        }
    };

    if let Err(e) = tun::setup_tun_iface() {
        println!("Failed to setup TUN interface: {}", e);
        return;
    }
    println!("Successfully assigned IP to interface");

    let shared_tun = Arc::new(Mutex::new(tun_iface));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client with IP: {} connected successfully", stream.peer_addr().unwrap());
                let tun_iface_clone = Arc::clone(&shared_tun);

                thread::spawn(move || {
                    let mut locked_tun = tun_iface_clone.lock().unwrap();
                    read_from_tun_and_send_to_client(&mut *locked_tun, stream);
                });
            },
            Err(err) => {
                println!("Connection failed: {}", err);
            }
        }
    }
}

fn create_listener(ip: String) -> Result<TcpListener, io::Error> {
    TcpListener::bind(ip)
}
