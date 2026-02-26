# Rust System Monitor 📊

A real-time system resource monitor CLI tool built with Rust. It provides a clear and concise view of your system's CPU and memory usage, along with basic system information.

## Features

- **Real-time Monitoring:** Updates system resource usage at a configurable interval.
- **CPU Usage:** Displays usage for each CPU core with a visual progress bar.
- **Memory Usage:** Shows total and used memory (RAM and Swap) with a visual progress bar.
- **System Information:** Displays system name, kernel version, OS version, and host name.
- **Configurable Refresh:** Set the refresh interval and the number of updates.

## Installation

To build the tool from source, you need to have Rust and Cargo installed.

```bash
git clone https://github.com/rawqubit/rust-sys-monitor.git
cd rust-sys-monitor
cargo build --release
```

The binary will be available at `target/release/rust-sys-monitor`.

## Usage

```bash
rust-sys-monitor [OPTIONS]
```

### Options

- `-i, --interval <INTERVAL>`: Refresh interval in seconds (default: `2`).
- `-c, --count <COUNT>`: Number of times to refresh (0 for infinite, default: `0`).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Examples

Monitor system resources with a 1-second refresh interval:
```bash
rust-sys-monitor -i 1
```

Refresh 5 times and then exit:
```bash
rust-sys-monitor -c 5
```


