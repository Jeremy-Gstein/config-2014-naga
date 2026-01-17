use crate::key_map::KeyMapper;
use crate::naga::Naga;
use evdev_rs::enums::EventCode::{EV_KEY, EV_SYN};
use evdev_rs::InputEvent;
use uinput::device::Device;
use uinput::Error;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

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

pub fn map_events(
    key_mapper: KeyMapper,
    naga: Naga,
    device: &mut Device,
    running: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Check if we should stop
        if !running.load(Ordering::SeqCst) {
            #[cfg(debug_assertions)]
            eprintln!("Detached from naga");
            break;
        }

        // Try to read event (non-blocking now)
        match naga.next_event() {
            Ok((_read_status, input_event)) => {
                process_event(key_mapper, input_event, device)
                    .map_err(|e| format!("Process event error: {}", e))?;
            }
            Err(e) => {
                // Check if it's a "would block" error (no data available)
                if e.contains("Resource temporarily unavailable") || e.contains("EAGAIN") {
                    // No data available, sleep briefly and check running flag again
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    continue;
                } else {
                    // Got conditions for real error
                    return Err(e.into());
                }
            }
        }
    }
    
    Ok(())
}

fn process_event(
    key_mapper: KeyMapper,
    event: InputEvent,
    input_device: &mut Device,
) -> Result<(), Error> {
    match event.event_code {
        EV_KEY(key) => {
            // Map hardware event codes to key mapper indices
            // Naga 2014 side buttons send codes 2-13 (corresponding to 1-0,-,= keys)
            // Convert EV_KEY enum to its numeric code
            let key_code = key as u32;
            let key_index = match key_code {
                2..=13 => Some((key_code - 2) as usize),  // Keys 1-12 map to indices 0-11
                _ => None,
            };
            
            if let Some(index) = key_index {
                if let Some(mapped_key) = key_mapper.keys.get(index).copied() {
                    #[cfg(debug_assertions)]
                    let action = match event.value {
                        1 => "PRESSED",
                        0 => "RELEASED",
                        _ => "UNKNOWN",
                    };
                    
                    debug_println!(
                        "Button {} (index {}) {} -> Key: {}",
                        index + 1,
                        index,
                        action,
                        mapped_key.debug_name()
                    );
                    
                    match event.value {
                        1 => input_device.press(&mapped_key)?,
                        0 => input_device.release(&mapped_key)?,
                        _ => (),
                    }
                } else {
                    debug_println!("No mapped key for button {} (index {})", index + 1, index);
                }
            }
        }
        EV_SYN(_) => input_device.synchronize()?,
        _ => (),
    };
    Ok(())
}
