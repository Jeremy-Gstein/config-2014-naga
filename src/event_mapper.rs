use super::key_map::KeyMapper;
use super::naga::Naga;
use evdev_rs::enums::EventCode::{EV_KEY, EV_SYN};
use evdev_rs::InputEvent;
use uinput::device::Device;
use uinput::Error;

pub fn map_events(
    key_mapper: KeyMapper,
    naga: Naga,
    input_device: &mut Device,
) -> Result<(), String> {
    loop {
        let (_read_status, input_event) = naga
            .next_event()
            .map_err(|e| format!("Event read error: {}", e))?;

        process_event(key_mapper, input_event, input_device)
            .map_err(|e| format!("Process event error: {}", e))?
    }
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
                    match event.value {
                        1 => input_device.press(&mapped_key)?,
                        0 => input_device.release(&mapped_key)?,
                        _ => (),
                    }
                }
            }
        }
        EV_SYN(_) => input_device.synchronize()?,
        _ => (),
    };
    Ok(())
}
