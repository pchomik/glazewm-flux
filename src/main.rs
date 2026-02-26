// #![windows_subsystem = "windows"]

mod actions;
mod config;
mod hook;
mod keybindings;
pub mod window;
pub mod workspace;
use std::sync::mpsc;

use config::Config;
use tray_item::{IconSource, TrayItem};

fn main() {
    let config_path = dirs::config_dir()
        .map(|p| p.join("GlazeWM-Flux").join("config.toml"))
        .expect("Could not find config directory");

    let config = if config_path.exists() {
        Config::from_file(&config_path).expect("Failed to load config")
    } else {
        // Fallback or exit
        eprintln!("Config file not found: {:?}", config_path);
        std::process::exit(1);
    };

    let mut tray = TrayItem::new("GlazeWM Flux", IconSource::Resource("main-icon")).unwrap();
    tray.add_label("GlazeWM Flux").unwrap();
    tray.add_menu_item("Quit", || {
        std::process::exit(0);
    })
    .unwrap();

    let parsed_bindings = match keybindings::parse_all(&config.keybindings) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error parsing keybindings: {}", e);
            std::process::exit(1);
        }
    };

    let (sender, receiver) = mpsc::channel::<String>();
    let config_clone = config.clone();

    // Span a thread to process commands received from the hook
    std::thread::spawn(move || {
        while let Ok(cmd) = receiver.recv() {
            actions::execute(&cmd, &config_clone);
        }
    });

    // Start the hook on the main thread (blocks and runs GetMessage loop)
    if let Err(e) = hook::start_hook(parsed_bindings, sender) {
        eprintln!("Failed to start keyboard hook: {}", e);
        std::process::exit(1);
    }
}
