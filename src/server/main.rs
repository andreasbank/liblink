use std::sync::{Mutex};
use std::error::Error;
//use debug_print::debug_println;
use clap::{arg, command};
use once_cell::sync::Lazy;

use liblink::common::verbose_print::{VerbosityLevel, Verbosity};
use liblink::{Error, Inform, Detail, Spam};
mod server;
use server::LinkServer;

static VERBOSITY: Lazy<Mutex<Verbosity>> = Lazy::new(|| Mutex::new(Verbosity::new()));

fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = LinkServer::new(&"127.0.0.1".to_string(), 65432).unwrap();
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

    VERBOSITY.lock().unwrap().set_level(VerbosityLevel::Spam);

    let _listen_conf = match cmd_args.get_one::<String>("listen") {
        Some(conf) => String::from(conf),
        None => String::from(""),
    };

    Detail!("Parsed conf: '{}')", _listen_conf);

    println!("Listening on port 65432");
    listener.start()?;

    // This will never be reached
    listener.stop();

    Ok(())
}
