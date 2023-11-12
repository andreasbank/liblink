use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_accept(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                /* echo the data */
                stream.write(&data[0..size]).unwrap();
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1";
    let port = 65432;
    let listener = TcpListener::bind((addr, port)).unwrap();

    println!("Listening on port 65432");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection from address {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_accept(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}