use evdev_rs::{Device, GrabMode, InputEvent, ReadStatus, ReadFlag};
use std::fs::{read_dir, File};
use std::error::Error;
use std::os::unix::io::AsRawFd;

pub struct Naga {
    device: Device,
    // need to keep this file, otherwise file would be closed too early
    _file: File,
}

impl Naga {
    pub fn new() -> Result<Naga, Box<dyn Error>> {
        let paths = read_dir("/dev/input")
            .map_err(|e| format!("Problem reading input devices dir: {}", e))?;

        for path_result in paths {
            let path = match path_result {
                Ok(p) => p,
                Err(_) => {
                    continue;
                }
            };

            let file = File::open(path.path())?;            
            let file_clone = file.try_clone()?;

            let mut device = Device::new_from_fd(file)?;

            if device.name().unwrap_or("").eq("Razer Razer Naga 2014")
                && device.phys().unwrap_or("").ends_with("/input2")
            {
                device
                    .grab(GrabMode::Grab)
                    .map_err(|e| format!("Could not grab device: {}", e))?;
                
                // Set the device to non-blocking mode
                let fd = file_clone.as_raw_fd();
                unsafe {
                    let flags = libc::fcntl(fd, libc::F_GETFL);
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
                }
                
                return Ok(Naga {
                    device,
                    _file: file_clone,
                });
            }
        }

        Err("No device found".to_string())?
    }

    pub fn next_event(&self) -> Result<(ReadStatus, InputEvent), String> {
        match self.device.next_event(ReadFlag::NORMAL) {
            Ok(res) => Ok(res),
            Err(errno) => Err(format!("Problem reading event: {}", errno)),
        }
    }
}
