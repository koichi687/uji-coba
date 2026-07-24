use std::net::{TcpListener};

fn main() {
    let listen = TcpListener::bind("12.0.0.1:2021")
        .expect("gagal");

    println!("waiting for..");

    for stream in listen.incoming() {
        match stream {
            Ok(_stream) => {
                println!("yay")
            },
            Err(_) => {
                println!("eror");
                println!("waiting for cancle");
                break;
        }
        }
    }

}
