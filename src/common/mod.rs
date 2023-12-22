use std::sync::{Mutex, Arc};

/*
// Create a global state wrapped in a Mutex
static VERBOSITY: Mutex<Verbosity> = Mutex::new(Verbosity::new)));

    // Set the global state flag to true
    VERBOSITY.lock().unwrap().set_flag(true);

    // Use the macro to conditionally print text
    conditional_print!("This text will be printed because the global flag is set. {}", 42);
*/

// Define a struct to encapsulate the global state
pub struct Verbosity{
    flag: bool,
}

// Implement methods for the struct to safely modify the state
impl Verbosity {
    pub fn new() -> Self {
        Verbosity { flag: false }
    }

    pub fn set_flag(&mut self, value: bool) {
        self.flag = value;
    }

    pub fn get_flag(&self) -> bool {
        self.flag
    }
}

// Define the macro to accept arguments like println!
#[macro_export]
macro_rules! conditional_print {
    ($($arg:tt)*) => {
        {
            // Check the global state
            if VERBOSITY.lock().unwrap().get_flag() {
                // Print the text with the same arguments as println!
                println!($($arg)*);
            }
        }
    };
}
