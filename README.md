# rpomo

A minimal Pomodoro timer for Linux, built in Rust with zero crate dependencies.

## Features

- Configurable session count, session length, and break time
- Desktop notifications via notify-send
- GUI prompts via zenity
- Descending session times (25 → 20 → 15 → 10 → 5)

## Requirements

- `notify-send` — desktop notifications
- `zenity` — GTK dialog prompts

```bash
# Arch Linux
sudo pacman -S libnotify zenity

# Ubuntu / Debian
sudo apt install libnotify-bin zenity

# Fedora
sudo dnf install libnotify zenity

# openSUSE
sudo zypper install libnotify-tools zenity
```

## Install

```bash
cargo install rpomo
```

## Or build from source

```bash
cargo build --release
./target/release/rpomo
```

## License

MIT — Mohamed Gonem 2026
