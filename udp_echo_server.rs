use std::thread;
use std::net::UdpSocket;

fn main() {

    let socket = UdpSocket::bind("0.0.0.0:8888").expect("cant bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("failed to clone socket oh no");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("handlin connection from {}", src);
                    sock.send_to(&buf, &src).expect("failed to send a response");
                });
            },
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}

