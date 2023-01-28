use serde::Serialize;
use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::process;
use std::thread;

#[derive(Debug)]
enum HyprReadErrors {
    InstanceSignatureEnvVarMissing,
}

impl std::fmt::Display for HyprReadErrors {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("{self}")
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    read_path: String,
}

impl Config {
    fn build() -> Result<Config, HyprReadErrors> {
        match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
            Ok(instance_sig) => Ok(Config {
                read_path: format!("/tmp/hypr/{instance_sig}/.socket2.sock"),
            }),
            Err(_) => Err(HyprReadErrors::InstanceSignatureEnvVarMissing),
        }
    }
}

#[derive(Serialize, Debug)]
struct Status {
    text: String,
}

impl std::fmt::Display for Status {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&serde_json::to_string(self).unwrap())
    }
}

pub fn handle_event(event: String) {
    if event.starts_with("activewindow") {
        let vec: Vec<&str> = event.split(',').collect();
        let mut text = vec.last().unwrap().to_string();

        if !text.is_empty() {
            text.truncate(64);

            let active_window = Status { text };

            println!("{active_window}");
        }
    }
}

pub fn start_client() -> std::io::Result<()> {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Environment error {err}");
        process::exit(1);
    });
    let socket = UnixStream::connect(config.read_path).unwrap_or_else(|err| {
        eprintln!("Could not connect to Hyprland: {err}");
        process::exit(1);
    });
    let reader = BufReader::new(socket);

    for event in reader.lines().flatten() {
        thread::spawn(|| handle_event(event));
    }

    Ok(())
}
