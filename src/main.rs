//! Config 2014 Naga - Linux CLI tool
//!
//! Maps the 12 side buttons on the Razer Naga 2014 mouse to configurable keyboard keys.
//!
//! # Usage
//!
//! Run with default key mapping (1-0, minus, equal):
//! ```bash
//! config-2014-naga
//! ```
//!
//! Run with custom TOML config file:
//! ```bash
//! config-2014-naga config.toml
//! ```
//!
//! # Configuration
//!
//! Create a TOML file to customize key mappings:
//! ```toml
//! [keys]
//! "1" = "F1"
//! "2" = "F2"
//! "3" = "LeftShift"
//! ```

use std::env;
use std::error::Error;
use config_2014_naga::{key_map::KeyMapper, run_loop_blocking};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

macro_rules! debug_println {
    ($($arg:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                eprintln!($($arg)*);
            }
        }
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}-v{}", NAME, VERSION);

    let args: Vec<String> = env::args().collect();

    let (key_mapper, config_source) = match args.len() {
        2 => {
            let mapper = KeyMapper::read_from_file(&args[1])?;
            (mapper, format!("file: {}", args[1]))
        },
        1 => (KeyMapper::default(), "default".to_string()),
        _ => return Err("Too many arguments".into()),
    };

    println!("Configuration loaded from: {}", config_source);
    debug_println!("\nKey mappings:");
    debug_println!("{}", key_mapper.debug_mappings());

    // Run indefinitely
    run_loop_blocking(key_mapper)
}
