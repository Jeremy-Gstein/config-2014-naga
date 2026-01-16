//! Config 2014 Naga Library
//!
//! This library provides functionality to remap the 12 side buttons on a
//! Razer Naga 2014 mouse to configurable keyboard keys on Linux.
//!
//! # Example
//!
//! ```no_run
//! use config_2014_naga::{naga::Naga, key_map::KeyMapper, input_device, event_mapper};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load key mapping
//! let key_mapper = KeyMapper::default();
//!
//! // Connect to device
//! let naga = Naga::new()?;
//! let mut device = input_device::create()?;
//!
//! // Start mapping
//! event_mapper::map_events(key_mapper, naga, &mut device)?;
//! # Ok(())
//! # }
//! ```

pub mod event_mapper;
pub mod input_device;
pub mod key_map;
pub mod naga;
