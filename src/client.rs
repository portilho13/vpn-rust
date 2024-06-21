use std::net::TcpStream;
use std::io::{Error, Read, Write};
use crate::tun;
use ::tun::Device;

pub fn client(ip: String) {
    let mut client = match create_client(ip) {
        Ok(client) => {
            println!("Successfully connected to IP: {}", client.peer_addr().unwrap());
            client
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let mut tun_iface = match tun::create_tun_iface() {
        Ok(t) => {
            println!("Iface {} created successfully", t.name().unwrap());
            t
        },
        Err(e) => {
            println!("Error creating Iface: {}", e);
            return;
        }
    };

    tun::setup_client_tun_iface();
    println!("Successfully assigned IP to interface");

    let mut buffer = [0u8; 1500];

    loop {
        match tun_iface.read(&mut buffer) {
            Ok(n) => {
                println!("Readed {} bytes from Iface", n);
                let packet = tun::Packet { data: buffer[..n].to_vec() };
                let serialized_packet = bincode::serialize(&packet).unwrap();
                
                if let Err(e) = client.write_all(&serialized_packet) {
                    println!("Failed to send packet to server: {}", e);
                    break;
                }
            },
            Err(e) => {
                println!("Failed to read from iface {}", e);
            }
        }
        match client.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Connection closed by server");
                    break;
                }
                println!("Read {} bytes from server", n);
                let packet: tun::Packet = bincode::deserialize(&buffer[..n]).unwrap();
                println!("Tun data from server: {:?}", packet.data);

            }
            Err(e) => {
                println!("Error reading from server: {}", e);
                break;
            }
        }
    }
}

fn create_client(ip: String) -> Result<TcpStream, Error> {
    TcpStream::connect(ip)
}
