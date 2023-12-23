use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::result::Result;
use std::sync::{Mutex};
use debug_print::debug_println;
use clap::{arg, command};
use once_cell::sync::Lazy;

use liblink::common::utils::encode_hex;
use liblink::common::verbose_print::Verbosity;
use liblink::verb0;

static VERBOSITY: Lazy<Mutex<Verbosity>> = Lazy::new(|| Mutex::new(Verbosity::new()));

static MAGIC_BYTES: [u8; 4] = [ 0xAB, 0xBA, 0xAB, 0xBA ];

fn authenticate(data: &[u8]) -> Result<(), &'static str> {
    debug_println!("Magic bytes received: {}", encode_hex(data));
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

    VERBOSITY.lock().unwrap().set_flag(true);

    let cmd_args = command!().args(&[
        arg!(-d --debug <lvl> "Enable debug at a certain level"),
        arg!(-v --verbose ... "Verbose mode. Can be specified up to 3 times"),
        arg!(--listen <string> "Start listening for multicast data"),
    ]).get_matches();

    // Remove'_' from _dbg_lvl when this is used
    let _dbg_lvl = match cmd_args.get_one::<String>("debug") {
        Some(v) => match String::from(v).trim().parse() {
            Ok(n) => {
                println!("Debug is enabled (level {})", n);
                n
            },
            Err(_e) => 0, // Move this error checking to the command!().
        },
        None => 0,
    };

    let _listen_conf = match cmd_args.get_one::<String>("listen") {
        Some(conf) => String::from(conf),
        None => String::from(""),
    };

    verb0!("Hello (conf: '{}')", _listen_conf);

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
