# luma

An ultra-lightweight, resource-efficient screen color temperature daemon written in Rust. It automatically adjusts your display's color temperature throughout the day with smooth transitions while running in the background with near-zero CPU usage.

---

## 🚀 Compatibility

`luma` automatically detects your display environment and uses the most appropriate backend:

| Environment | Backend |
|------------|---------|
| **Wayland (Hyprland)** | High-performance GLSL 300 ES shader injection via `hyprctl` |
| **Wayland (KDE Plasma)** | Native D-Bus integration with KWin Night Light |
| **X11** | Native X11 (`x11rb`) + RandR hardware gamma control |

---

## 🛠️ Installation

### Clone the repository

```bash
git clone https://github.com/hakuxyz/luma.git
cd luma
```

### Build

Make sure the Rust toolchain is installed, then run:

```bash
cargo build --release
```

### Install

Copy the binary into your `$PATH`:

```bash
sudo install -Dm755 target/release/luma /usr/bin/luma
```

Reload the user systemd daemon:

```bash
systemctl --user daemon-reload
```

### Enable the service

```bash
systemctl --user enable --now luma
```

---

## ⚙️ Configuration

`luma` reads its configuration from:

```text
~/.config/luma/config.toml
```

Example:

```toml
day_temp = 6500
night_temp = 3500
start_time = "20:00"
end_time = "06:00"
transition_mins = 60
```

---

## 📊 Useful Commands

### View logs

```bash
journalctl --user -u luma -f
```

### Restart

```bash
systemctl --user restart luma
```

### Stop

```bash
systemctl --user stop luma
```

---

## 🐛 Known Issues

> **⚠️ Beta Software**
>
> `luma` is still under active development. Bugs and unexpected behavior are possible, particularly on uncommon desktop environments.

### Wayland backend detection

To work around systemd user services not always inheriting the correct display environment, `luma` attempts multiple Wayland backends. In unusual or nested compositor setups, this may produce unexpected behavior.

### Missing display environment

If your user systemd instance strips environment variables such as `WAYLAND_DISPLAY` or `HYPRLAND_INSTANCE_SIGNATURE`, `luma` may be unable to locate the active display session.

### Hyprland shader compatibility

The Hyprland backend relies on injecting a custom GLSL shader. Future Hyprland releases that change internal shader interfaces may temporarily break compatibility until `luma` is updated.

If shader compilation fails, check the logs:

```bash
journalctl --user -u luma -f
```
