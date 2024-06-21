use std::{error::Error, io::Write, process::Command};
use serde_derive::{Serialize, Deserialize};
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub data: Vec<u8>,
}

const TUN_INTERFACE: &str = "tun0";

pub fn create_tun_iface() -> Result<tun::platform::Device, tun::Error> {
    let mut config = tun::Configuration::default();
    config.name(TUN_INTERFACE);
    config.up();
    tun::create(&config)
}

pub fn setup_tun_iface() -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(TUN_INTERFACE)
        .arg("up")
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to set up {}: {:?}", TUN_INTERFACE, output.stderr).into());
    }

    let output = Command::new("sudo")
        .arg("ip")
        .arg("addr")
        .arg("add")
        .arg("10.0.0.5/24")
        .arg("dev")
        .arg(TUN_INTERFACE)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to assign IP to {}: {:?}", TUN_INTERFACE, output.stderr).into());
    }

    Ok(())
}

pub fn setup_client_tun_iface() {
    let ip_output = Command::new("ip")
        .arg("addr")
        .arg("add")
        .arg("10.0.0.2/24")
        .arg("dev")
        .arg("tun0")
        .output()
        .expect("Failed to execute IP command");

    if !ip_output.status.success() {
        eprintln!("Failed to set IP: {}", String::from_utf8_lossy(&ip_output.stderr));
        return;
    }

    let link_output = Command::new("ip")
        .arg("link")
        .arg("set")
        .arg("up")
        .arg("dev")
        .arg("tun0")
        .output()
        .expect("Failed to execute IP LINK command");

    if !link_output.status.success() {
        eprintln!("Failed to set link up: {}", String::from_utf8_lossy(&link_output.stderr));
        return;
    }

    let route_output = Command::new("ip")
        .arg("route")
        .arg("add")
        .arg("0.0.0.0/0")
        .arg("via")
        .arg("10.8.0.1")
        .arg("dev")
        .arg("tun0")
        .output()
        .expect("Failed to execute IP ROUTE command");

    if !route_output.status.success() {
        eprintln!("Failed to set route: {}", String::from_utf8_lossy(&route_output.stderr));
    }
}

pub fn read_from_tun_and_send_to_client<T: tun::Device>(tun: &mut T, mut client: TcpStream) {
    let mut buffer = [0u8; 1500];

    loop {
        match tun.read(&mut buffer) {
            Ok(n) => {
                println!("Read {} bytes from iface", n);
                let packet = Packet { data: buffer[..n].to_vec() };
                println!("Data: {:?}", packet.data);
                let serialized_packet = bincode::serialize(&packet).unwrap();
                if let Err(e) = client.write_all(&serialized_packet) {
                    println!("Failed to send packet to client: {}", e);
                    break;
                }
            },
            Err(e) => {
                println!("Failed to read from iface: {}", e);
                break;
            }
        }
    }
}
