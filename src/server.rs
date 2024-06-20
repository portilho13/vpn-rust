use std::{io, net::TcpListener};
use ::tun::Device;
use std::sync::{Mutex, Arc};


use crate::tun::{self, read_from_tun_and_send_to_client};

pub fn server(ip: String) {
    let listener = match create_listener(ip.clone()) {
        Ok(t) => {
            println!("Started to listen on ip {}", t.local_addr().unwrap());
            t
        },
        Err(err) => {
            println!("Error: {}", err); 
            return
        }
    };

    let tun_iface = match tun::create_tun_iface() {
        Ok(t) => {
            println!("Iface {} created sucessfully", t.name().unwrap());
            t
        },
        Err(e) => {
            println!("Error creating Iface: {}", e);
            return;
        }
    };

    if let Err(e) = tun::setup_tun_iface(ip.as_str()) {
        println!("Failed to setup tun interface: {}", e);
        return;
    }
    println!("Sucessfully adressed IP to interface");

    let shared_tun = Arc::new(Mutex::new(tun_iface));

    let tun_iface_clone = shared_tun.clone();

    let mut locked_tun = tun_iface_clone.lock().unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client with IP: {} connected sucessfully", stream.peer_addr().unwrap());
                read_from_tun_and_send_to_client(&mut *locked_tun, stream);
            },
            Err (err    ) => {
                println!("Connection failed: {}", err)
            }
        }
    }
}

fn create_listener(ip: String) -> Result<TcpListener, io::Error> {
    let listener = TcpListener::bind(ip)?;
    Ok(listener)
}