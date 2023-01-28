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
        let text: &str = event.split(',').collect::<Vec<&str>>()[1];

        if !text.is_empty() {
            let active_window = Status {
                text: text.to_string(),
            };

            println!("{active_window}");
        }
    }
}

fn connect_to_hyprland(config: Config) -> Result<(), std::io::Error> {
    let socket: UnixStream = UnixStream::connect(config.read_path)?;
    let reader: BufReader<UnixStream> = BufReader::new(socket);

    for event in reader.lines().flatten() {
        thread::spawn(|| handle_event(event));
    }

    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Environment error {err}");
        process::exit(1);
    });

    connect_to_hyprland(config).unwrap_or_else(|err| {
        eprintln!("Could not connect to Hyprland: {err}");
        process::exit(1);
    });

    Ok(())
}
