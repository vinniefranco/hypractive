# Hypractive

A waybar utility written in rust that displays the current `activewindow`

### Building

```bash
$ cargo build --release
$ cp target/release/hyperactive <your_waybar_scripts_directory>
```

### Using in waybar

#### 1. Define a custom module

```json
{
  // Hypractive
    "custom/hypractive": {
      "exec": "~/.config/hypr/scripts/hypractive 2> /dev/null",
      "exec-if": "pgrep hypractive",
      "format": "{}",
      "return-type": "json",
      "max-length": 64,
    },
}

```

#### 2. Add the custom module to your main config

```json
{
  "module-center": ["custom/hypractive"]
}
```
