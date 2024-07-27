use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8854").expect("to connect");

    dbg!("connected");

    stream.write(b"hello").expect("write to work");

    let mut buffer = [0; 1024];
    loop {
        let read = stream.read(&mut buffer).expect("it to work");
        if read > 0 {
            dbg!(read);
        }
    }
}
