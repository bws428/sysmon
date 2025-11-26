use sysinfo::{Components, Disks, Networks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\nSystem Monitor v0.1\n\n");

    // CPU info
    println!("=== CPU ===");
    println!("\nCPU count: {}", sys.cpus().len());
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
}
