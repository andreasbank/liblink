
/*
USAGE:

use std::sync::{Mutex, Arc};

// Create a global state wrapped in a Mutex
static VERBOSITY: Mutex<Verbosity> = Mutex::new(Verbosity::new)));

    // Set the global state flag to true
    VERBOSITY.lock().unwrap().set_flag(true);

    // Use the macro to conditionally print text
    verb0!("This text will be printed because the global flag is set. {}", 42);
*/

#[derive(PartialEq, PartialOrd)]
pub enum VerbosityLevel {
    Quiet,
    Errors,
    Informative,
    Detailed,
    Spam
}

// Define a struct to encapsulate the global state
pub struct Verbosity{
    level: VerbosityLevel,
}

// Implement methods for the struct to safely modify the state
impl Verbosity {
    pub fn new() -> Self {
        Verbosity { level: VerbosityLevel::Quiet }
    }

    pub fn set_level(&mut self, value: VerbosityLevel) {
        self.level = value;
    }

    pub fn is_atleast_level(&self, level: VerbosityLevel) -> bool {
        if level <= self.level {
            return true;
        }
        false
    }
}

// Define the macros to accept arguments like println!
#[macro_export]
macro_rules! Error {
    ($($arg:tt)*) => {
        {
            // Check the global state
            if VERBOSITY.lock().unwrap().is_atleast_level(VerbosityLevel::Errors) {
                println!($($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! Inform {
    ($($arg:tt)*) => {
        {
            // Check the global state
            if VERBOSITY.lock().unwrap().is_atleast_level(VerbosityLevel::Informative) {
                println!($($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! Detail {
    ($($arg:tt)*) => {
        {
            // Check the global state
            if VERBOSITY.lock().unwrap().is_atleast_level(VerbosityLevel::Detailed) {
                println!($($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! Spam {
    ($($arg:tt)*) => {
        {
            // Check the global state
            if VERBOSITY.lock().unwrap().is_atleast_level(VerbosityLevel::Spam) {
                println!($($arg)*);
            }
        }
    };
}
