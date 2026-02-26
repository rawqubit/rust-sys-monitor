use anyhow::Result;
use clap::Parser;
use colored::*;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Refresh interval in seconds
    #[arg(short, long, default_value = "2")]
    interval: u64,

    /// Number of times to refresh (0 for infinite)
    #[arg(short, long, default_value = "0")]
    count: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let mut current_count = 0;

    loop {
        sys.refresh_all();

        // Clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("{}", "--- System Resource Monitor ---".bold().cyan());
        println!("{}: {}", "System name".yellow(), System::name().unwrap_or_default());
        println!("{}: {}", "Kernel version".yellow(), System::kernel_version().unwrap_or_default());
        println!("{}: {}", "OS version".yellow(), System::os_version().unwrap_or_default());
        println!("{}: {}", "Host name".yellow(), System::host_name().unwrap_or_default());

        println!("\n{}", "--- CPU Usage ---".bold().green());
        for (i, cpu) in sys.cpus().iter().enumerate() {
            let usage = cpu.cpu_usage();
            let bar = get_usage_bar(usage);
            println!("CPU {}: {:>5.1}% {}", i, usage, bar);
        }

        println!("\n{}", "--- Memory Usage ---".bold().magenta());
        let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let mem_usage = if total_mem > 0.0 { (used_mem / total_mem) * 100.0 } else { 0.0 };
        let mem_bar = get_usage_bar(mem_usage as f32);
        println!("Memory: {:.2} GB / {:.2} GB ({:.1}%) {}", used_mem, total_mem, mem_usage, mem_bar);

        let total_swap = sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_swap = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
        if total_swap > 0.0 {
            let swap_usage = (used_swap / total_swap) * 100.0;
            let swap_bar = get_usage_bar(swap_usage as f32);
            println!("Swap:   {:.2} GB / {:.2} GB ({:.1}%) {}", used_swap, total_swap, swap_usage, swap_bar);
        }

        current_count += 1;
        if cli.count > 0 && current_count >= cli.count {
            break;
        }

        thread::sleep(Duration::from_secs(cli.interval));
    }

    Ok(())
}

fn get_usage_bar(usage: f32) -> String {
    let width = 20;
    let filled = (usage / 100.0 * width as f32) as usize;
    let empty = width - filled;

    let bar = format!(
        "[{}{}]",
        "|".repeat(filled).green(),
        ".".repeat(empty).white()
    );
    bar
}
