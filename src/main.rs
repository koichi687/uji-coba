use std::{io, net::{TcpListener, TcpStream}};
use std::io::{Write};

fn handle(stream: io::Result<TcpStream>) {
    match stream {
        Ok(mut stream) => {
            println!("hello");
            let body_test = "hello guy";
            let respon =  format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body_test.len(), body_test);
            stream.write_all(respon.as_bytes()).expect("err");
            stream.flush().expect("gagal flush");
            println!("response terkirim");
        },
        Err(_) => {
            println!("eror, repeat again")
        }
    }
}   

fn main() {
    let listen = TcpListener::bind("0.0.0.0:221").expect("eror");

    for stream in listen.incoming() {
        handle(stream);
    }
}

