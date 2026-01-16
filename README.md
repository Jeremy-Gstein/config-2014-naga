# config-2014-naga 

A Linux utility written in Rust to remap all 12 side buttons on the Razer Naga 2014 mouse to configurable keyboard and numpad keys.

## Features

- Maps all 12 Naga side buttons to any keyboard key
- Configurable via TOML files
- Default mapping to number row (1-0, minus, equal)
- Debug mode for troubleshooting
- Zero-overhead release builds

## Install
### [crates.io](https://crates.io/crates/config-2014-naga)
```bash
cargo install config-2014-naga
# run with default key mapping
config-2014-naga
# specifiy key mapping with ./config/config-2014-naga.toml
config-2014-naga /path/to/config-2014-naga.toml
```

### From Source
```bash
git clone https://github.com/Jeremy-Gstein/config-2014-naga
cd config-2014-naga
cargo build --release
cargo run --release
# build with verbos debug output
cargo build
cargo run
```

--- 

### Default Mapping

Run with the default key mapping (buttons 1-12 → keys 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, -, =):

```bash
sudo config-2014-naga
```

### Custom Mapping

Create a TOML configuration file:

```toml
# config.toml
[keys]
"1" = "F1"
"2" = "F2"
"3" = "F3"
"4" = "F4"
"5" = "F5"
"6" = "F6"
"7" = "LeftShift"
"8" = "LeftControl"
"9" = "LeftAlt"
"10" = "Space"
"11" = "KP::_1"
"12" = "KP::_2"
```

## Permissions

The program needs root access to:
- Read from `/dev/input` devices
- Create virtual keyboard via `/dev/uinput`

## Documentation

Generate and view the API documentation:

```bash
cargo doc --open
```

## How It Works

1. Scans `/dev/input` for "Razer Razer Naga 2014" device with physical path ending in "/input2"
2. Grabs exclusive access to the device (prevents default behavior)
3. Creates a virtual keyboard device via uinput
4. Reads events from the Naga side buttons
5. Maps button events to configured keys
6. Sends mapped key events to the virtual keyboard

## Troubleshooting

### Device not found

Make sure your Razer Naga 2014 is plugged in:

```bash
ls /dev/input/by-id/ | grep -i razer
```

### Permission denied

Run with sudo or add your user to the `input` group:

```bash
sudo usermod -a -G input $USER
# Log out and back in
```

### Keys not working

Run in debug mode to see what's happening:

```bash
cargo build
sudo ./target/debug/keymap-2014-naga
```

Press the side buttons and watch the output.

### Project Origin & Credits

This project is derived from earlier work in the community:

Original project by [jpodeszwik](https://github.com/jpodeszwik):
- https://github.com/jpodeszwik/razer-naga-2014-key-remap

Extended fork with numpad support by [industrylol](https://github.com/industrylol):
- https://github.com/industrylol/razer-naga-2014-key-remap

This repository represents an independently maintained continuation with significant modifications, cleanup, and ongoing support.
