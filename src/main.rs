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

use std::time::Duration;
use std::{env, thread};
use std::error::Error;
use config_2014_naga::{event_mapper, input_device, key_map, naga};

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

macro_rules! error_println {
    ($($arg:tt)*) => {
        {
            eprintln!($($arg)*);
        }
    };
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("{}-v{}", NAME, VERSION);

    let args: Vec<String> = env::args().collect();

    let (key_mapper, config_source) = match args.len() {
        2 => {
            let mapper = key_map::KeyMapper::read_from_file(args[1].as_str())?;
            (mapper, format!("file: {}", args[1]))
        },
        1 => (key_map::KeyMapper::default(), "default".to_string()),
        _ => {
            return Err("Too many arguments")?;
        }
    };

    println!("Configuration loaded from: {}", config_source);
    debug_println!("\nKey mappings:");
    debug_println!("{}", key_mapper.debug_mappings());

    let mut device = input_device::create()?;

    loop {
        let naga = naga::Naga::new();

        match naga {
            Ok(dev) => {
                println!("Attached to naga");
                let res = event_mapper::map_events(key_mapper, dev, &mut device);
                if let Err(e) = res {
                    error_println!("Error mapping events: {}", e);
                } else {
                    debug_println!("Map events returned Ok which was not expected");
                }
            }
            Err(_err) => {
                debug_println!("Error looking for naga: {}", _err);
            }
        }
        thread::sleep(Duration::from_secs(1))
    }
}
