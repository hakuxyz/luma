mod backend;
mod config;
mod core;

use std::{env, fs, thread};
use std::time::Duration;
use std::io::{self, Write};
use chrono::Local;
use config::Config;
use backend::DisplayBackend;

fn main() {
    let config_path = dirs_next::home_dir().map(|p| p.join(".config/luma/config.toml")).unwrap_or_default();
    let config = if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    };

    // Robuste Prüfung: Wenn WAYLAND_DISPLAY existiert ODER hyprland läuft -> Wayland Modus
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok() || env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() || !env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().is_empty();

    if is_wayland {
        let mut backend = backend::wayland::WaylandBackend::connect().expect("Failed to init Wayland backend");
        println!("luma RUNNING IN WAYLAND MODE.");
        io::stdout().flush().unwrap();

        loop {
            let now = Local::now().time();
            let temp = core::calculate_current_temp(&config, now);
            let (r, g, b) = core::kelvin_to_rgb(temp);
            
            let _ = backend.set_gamma(temp, r, g, b);
            
            println!("Wayland Shader updated: {:.0}K -> R:{:.3} G:{:.3} B:{:.3}", temp, r, g, b);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    } else {
        let mut backend = backend::x11::X11Backend::connect().expect("Failed to init X11 backend");
        println!("luma RUNNING IN X11 MODE.");
        io::stdout().flush().unwrap();

        loop {
            let now = Local::now().time();
            let temp = core::calculate_current_temp(&config, now);
            let (r, g, b) = core::kelvin_to_rgb(temp);
            
            let _ = backend.set_gamma(temp, r, g, b);
            
            println!("X11 Gamma applied: {:.0}K -> R:{:.3} G:{:.3} B:{:.3}", temp, r, g, b);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    }
}
