use clap::Parser;
use std::fs::File;
use std::io::Write;
use sysinfo::{Disks, System};

/// A simple system monitor CLI
#[derive(Parser, Debug)]
#[command(name = "sysmon")]
#[command(about = "Monitor system resources", long_about = None)]
struct Args {
    /// Output for the report (optional)    
    #[arg(short, long)]
    output: Option<String>,

    /// Show only CPU information
    #[arg(long)]
    cpu: bool,

    /// Show only memory information
    #[arg(long)]
    mem: bool,

    /// Show only disk information
    #[arg(long)]
    disk: bool,
}

fn save_stats(sys: &System, filename: &str) -> std::io::Result<()> {
    // The ? operator: if an error occurs, return it immediately
    let mut file = File::create(filename)?;

    writeln!(file, "System Monitor Report")?;
    writeln!(file, "=====================\n")?;
    writeln!(file, "CPU Count: {}", sys.cpus().len())?;
    writeln!(
        file,
        "Total Memory: {} MB",
        sys.total_memory() / 1024 / 1024
    )?;
    writeln!(file, "Used Memory: {} MB", sys.used_memory() / 1024 / 1024)?;

    Ok(()) // Success with no meaningful return value
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\nSystem Monitor v0.1\n");

    // CPU info
    println!("=== CPU ===");
    println!("CPU count: {}", sys.cpus().len());
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("CPU {}: {}%", i, cpu.cpu_usage());
    }

    // Memory info
    println!("\n=== Memory ===");
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let free_mem = total_mem - used_mem;

    println!("Total: {} MB", total_mem / 1024 / 1024);
    println!(
        "Used:  {} MB ({:.1}%)",
        used_mem / 1024 / 1024,
        (used_mem as f64 / total_mem as f64) * 100.0
    );
    println!("Free:  {} MB", free_mem / 1024 / 1024);

    // Disk info
    let disks = Disks::new_with_refreshed_list();
    println!("\n=== Disks ===");
    for disk in &disks {
        println!("Drive: {}", disk.name().to_string_lossy());
        println!(" Mount: {}", disk.mount_point().display());
        println!(" Total: {} GB", disk.total_space() / 1024 / 1024 / 1024);
        println!(" Free: {} GB", disk.available_space() / 1024 / 1024 / 1024)
    }

    // Save stats to file
    match save_stats(&sys, "system_report.txt") {
        Ok(_) => println!("\n✓ Report saved!"),
        Err(e) => eprintln!("Failed to save report: {}", e),
    }
}
