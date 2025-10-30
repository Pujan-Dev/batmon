# Batmon ⚡

Batmon is a lightweight battery monitor and notification daemon for Linux desktops.  
It alerts you when your battery is full or critically low, complete with **desktop notifications and sound**, so you never get caught off guard. Perfect for Arch Linux, OMARCHY, Hyperland, and other minimal setups where default battery alerts are missing or silent.

---

## Features

- ✅ **Battery Full Notification** – alerts you when your battery reaches 100%.
- ✅ **Battery Low Notification** – warns you when your battery drops below 10%.
- ✅ **Sound Alerts** – each notification is accompanied by a sound.
- ✅ **Lightweight & Continuous** – runs in the background without consuming resources.
- ✅ **Works on Multiple Batteries** – averages battery percentage if your system has more than one.

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/Pujan-Dev/batmon
cd batmon
```

2. Build the project with Cargo:

```bash
cargo build --release
```

3. Run the program:

```bash
./target/release/batmon
```

> Optional: Run in background:

```bash
./target/release/batmon > /dev/null 2>&1 &
```

---

## Dependencies

- Rust 1.70+ (or latest stable)
- [`notify-rust`](https://crates.io/crates/notify-rust) crate
- `paplay` (for playing notification sounds, comes with `pulseaudio` or `pipewire-pulse`)
- Linux desktop environment that supports notifications

Install `paplay` on Arch/OMARCHY:

```bash
sudo pacman -S pulseaudio
```

---

## Usage

Batmon runs continuously, checking battery status every 60 seconds by default.
You will automatically get:

- **Full battery notifications** at 100%
- **Low battery notifications** at ≤10%

> You can customize the check interval and sound files by editing `main.rs`.

---

## Project Structure

```
batmon/
├─ src/
│  └─ main.rs        # main program
├─ Cargo.toml        # Rust configuration
└─ README.md
```

---

## Contributing

Contributions, suggestions, and bug reports are welcome!
Feel free to open issues or submit pull requests.

---

## License

This project is licensed under the MIT License.
Feel free to use and modify it freely.

---

## About

Batmon was created out of frustration with silent or missing battery notifications on lightweight Linux setups.
It’s now your **Battery Guardian Daemon** – keeping you informed while you focus on your work.

⚡ Stay charged. Stay notified. ⚡
