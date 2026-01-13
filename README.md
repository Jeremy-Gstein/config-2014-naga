# config-2014-naga 

A userspace key remapping utility for the **Razer Naga 2014** mouse, with extended support including **numpad-style mappings**.

This project is actively maintained and builds upon earlier community work to keep the Razer Naga 2014 usable on modern systems.

---

## Features
- Key remapping for Razer Naga 2014 with FULL support for side buttons 1-12
- Numpad-style button mappings
- Linux userspace implementation
- No kernel modules required [libevdev](https://github.com/ndesh26/evdev-rs)
- Lightweight and scriptable (configure with a small .toml file)
- Detailed runtime errors

### From Source
```bash
git clone https://github.com/Jeremy-Gstein/config-2014-naga
cd config-2014-naga
cargo build --release
```
--- 

### Project Origin & Credits

This project is derived from earlier work in the community:

Original project by [jpodeszwik](https://github.com/jpodeszwik):
- https://github.com/jpodeszwik/razer-naga-2014-key-remap

Extended fork with numpad support by [industrylol](https://github.com/industrylol):
- https://github.com/industrylol/razer-naga-2014-key-remap

This repository represents an independently maintained continuation with significant modifications, cleanup, and ongoing support.
