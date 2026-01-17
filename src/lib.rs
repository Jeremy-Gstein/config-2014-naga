//! Config 2014 Naga Library
//!
//! This library provides functionality to remap the 12 side buttons on a
//! Razer Naga 2014 mouse to configurable keyboard keys on Linux.
//!
//! # Example
//!
//! ```no_run
//! use config_2014_naga::{key_map::KeyMapper, run_loop};
//! use std::sync::{Arc, atomic::AtomicBool};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let key_mapper = KeyMapper::default();
//! let running = Arc::new(AtomicBool::new(true));
//! run_loop(key_mapper, running)?;
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
    let running = Arc::new(AtomicBool::new(true));
    event_mapper::map_events(key_mapper.clone(), naga, &mut device, running)?;
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
/// - Blocks until `running` is set to false
/// - Will exit cleanly within ~50ms of setting running to false
/// - CLI can pass `Arc::new(AtomicBool::new(true))` to run indefinitely
pub fn run_loop(key_mapper: KeyMapper, running: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
    let mut device = input_device::create()?;

    while running.load(Ordering::SeqCst) {
        match naga::Naga::new() {
            Ok(dev) => {
                #[cfg(debug_assertions)]
                eprintln!("Attached to naga");

                // Pass running flag so map_events can exit cleanly
                if let Err(e) = event_mapper::map_events(key_mapper.clone(), dev, &mut device, running.clone()) {
                    eprintln!("Error mapping events: {}", e);
                }
            }
            Err(_err) => {
                #[cfg(debug_assertions)]
                eprintln!("Error looking for naga: {}", _err);
            }
        }

        // Only sleep if still running (avoids delay on shutdown)
        if running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(1));
        }
    }

    #[cfg(debug_assertions)]
    eprintln!("run_loop exited cleanly");

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
