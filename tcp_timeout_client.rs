use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::net::SocketAddr;

fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("cant connect to server");
    stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Could not set a read timeout");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("failes to read from stdin");
        stream.write(input.as_bytes()).expect("failes to write to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer).expect("cant read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("cant not write buffer as string"));
    }
}
