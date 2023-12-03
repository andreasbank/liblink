use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::result::Result;

static MAGIC_BYTES: [u8; 4] = [ 0x0A, 0x0B, 0x0A, 0x0B ];

fn authenticate(data: &[u8]) -> Result<(), &'static str> {
    if MAGIC_BYTES == data { return Ok(()); }

    Err("Wrong magic bytes")
}

fn handle_accept(mut stream: TcpStream) {
    let mut authenticated = false;
    let mut data = [0 as u8; 1024];

    loop {
        match stream.read(&mut data) {

            Ok(size) if size > 0 => {

                /* Check for the magic bytes */
                if !authenticated {
                    match authenticate(&data[0..MAGIC_BYTES.len()]) {
                        Ok(_) => {
                            authenticated = true;
                            println!("Authentication successful");
                        }
                        Err(errstr) => {
                            println!("Failed to authenticate: {}", errstr);
                            break;
                        }
                    }
                }

                /* echo the data */
                println!("Sending data back");
                stream.write(&data[4..size]).unwrap();
            },

            /* If we received 0 bytes, we're done */
            Ok(_) => {
                println!("Gracefully closing the connection with {}", stream.peer_addr().unwrap());
                break;
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
