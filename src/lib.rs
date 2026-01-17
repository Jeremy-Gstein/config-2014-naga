//! Config 2014 Naga Library
//!
//! This library provides functionality to remap the 12 side buttons on a
//! Razer Naga 2014 mouse to configurable keyboard keys on Linux.
//!
//! # Example
//!
//! ```no_run
//! use config_2014_naga::{key_map::KeyMapper, run_loop};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let key_mapper = KeyMapper::default();
//! run_loop(key_mapper)?;
//! # Ok(())
//! # }
//! ```

pub mod event_mapper;
pub mod input_device;
pub mod key_map;
pub mod naga;

use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering}
};

use crate::key_map::KeyMapper;

/// Perform a single attach-and-map cycle.
///
/// This is useful for testing or higher-level control loops.
pub fn run_once(key_mapper: &KeyMapper) -> Result<(), Box<dyn Error>> {
    let mut device = input_device::create()?;
    let naga = naga::Naga::new()?;
    event_mapper::map_events(key_mapper.clone(), naga, &mut device)?;
    Ok(())
}

/// Run the naga remapper loop indefinitely.
///
/// # Arguments
///
/// * `key_mapper` - Key mapping to use
/// * `running` - Arc<AtomicBool> flag that allows graceful shutdown
///
/// # Notes
///
/// - Blocks forever unless `running` is set to false
/// - CLI can pass `Arc::new(AtomicBool::new(true))` to mimic old behavior
pub fn run_loop(key_mapper: KeyMapper, running: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
    let mut device = input_device::create()?;

    while running.load(Ordering::SeqCst) {
        match naga::Naga::new() {
            Ok(dev) => {
                #[cfg(debug_assertions)]
                eprintln!("Attached to naga");

                if let Err(e) = event_mapper::map_events(key_mapper.clone(), dev, &mut device) {
                    eprintln!("Error mapping events: {}", e);
                }
            }
            Err(_err) => {
                #[cfg(debug_assertions)]
                eprintln!("Error looking for naga: {}", _err);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

/// Backward-compatible version of `run_loop` for CLI usage
///
/// This simply creates a `running` flag that is always true, so the loop
/// never exits.
pub fn run_loop_blocking(key_mapper: KeyMapper) -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    run_loop(key_mapper, running)
}
