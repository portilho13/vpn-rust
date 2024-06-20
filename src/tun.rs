use std::io::Write;
use std::{error::Error, process::Command};
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
struct Packet {
    data: Vec<u8>
}

const TUN_INTERFACE: &str = "tun0";

pub fn create_tun_iface() -> Result<tun::platform::Device, tun::Error> {
    let mut config = tun::Configuration::default();

    config.name(TUN_INTERFACE);

    config.up();

    tun::create(&config)
}


pub fn setup_tun_iface(ip: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sudo")
    .arg("ip")
    .arg("link")
    .arg("set")
    .arg("dev")
    .arg(TUN_INTERFACE)
    .arg("up")
    .output()?;

    if !output.status.success() {
        return Err(format!("Failed to setup {}: {:?}", TUN_INTERFACE, output.stderr).into());
    }

    let ip: Vec<&str> = ip.split(":").collect();

    let ip = ip[0];



    let subnet = format!("{}/24", ip);

    let output = Command::new("sudo")
    .arg("ip")
    .arg("addr")
    .arg("add")
    .arg(subnet)
    .arg("dev")
    .arg(TUN_INTERFACE)
    .output()?;

    if !output.status.success() {
        return Err(format!("Failed to assign IP {} to {}: {:?}", ip, TUN_INTERFACE, output.stderr).into());
    }

    Ok(())

}

pub fn read_from_tun_and_send_to_client<T: tun::Device> (tun: &mut T, mut client: TcpStream) {
    let mut buffer = [0u8; 1500];

    loop {
        match tun.read(&mut buffer) {
            Ok(n) => {
                println!("Readed {} bytes from iface", n);
                let packet = Packet{ data : buffer[..n].to_vec() };
    
                let serialized_packet = bincode::serialize(&packet).unwrap();
                client.write_all(&serialized_packet).unwrap();
    
            },
            Err(e) => {
                println!("Failed to read from iface: {}", e);
                return
            }
        }
    }
}