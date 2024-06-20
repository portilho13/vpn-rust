mod args;
mod server;
mod client;
mod tun;

fn main() {
    let args = args::get_args();

    let mode = args[0].to_string();
    let ip = args[1].to_string();

    if mode == "server" {
        server::server(ip)
    } else if mode == "client" {
        client::client(ip)
    }

}
