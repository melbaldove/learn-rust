use std::net::TcpListener;

fn main() {
    TcpListener::bind("127.0.0.1:1337").map_err(|err} {
        eprintln!("ERROR: could not bind {address}");
    })
}
