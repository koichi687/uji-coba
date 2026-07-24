use std::net::{TcpListener};
use std::io::{Read, Write};

fn main() {
//TODO: local host, ganti ke port komputer masing masing yah
// 
    let listen = TcpListener::bind("0.0.0.0:8080")
        .expect("gagal");

    println!("waiting for..");

    for stream in listen.incoming() {
       let mut stream =  match stream {
            Ok(stream) => {
                println!("yay");
                stream
            },
            Err(_) => {
                println!("eror");
                println!("waiting for cancle");
                break;
            }
        };
        
        let mut now = [0; 1024];

        let test = stream.read(&mut now).unwrap();
        

        let pesan = String::from_utf8_lossy(&now[..test]);
        println!("something : ");
        println!("{}", pesan);

        stream.write_all(b"hallo").unwrap();
    }

}
