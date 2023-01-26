use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::thread;
use serde::Serialize;

#[derive(Clone, Debug)]
struct Config {
    read_path: String,
}

#[derive(Serialize, Debug)]
struct Status {
    text: String
}

impl std::fmt::Display for Status {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug)]
enum HyprReadErrors {
    InstanceSignatureEnvVarMissing,
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


fn handle_event(event: String) {
    let vec: Vec<&str> = event.split(',').collect();
    let mut text = vec.last().unwrap().to_string();
    text.truncate(64);

    let active_window = Status { text };

    println!("{active_window}");
}

fn main() -> std::io::Result<()> {
    let config = Config::build().expect("could not locate Hyprland instance");
    let socket = UnixStream::connect(config.read_path).expect("could not connect to Hyprland instance");
    let reader = BufReader::new(socket);

    for line in reader.lines() {
        if let Ok(event) = line {
            thread::spawn(|| handle_event(event));
        }
    }

    Ok(())
}
