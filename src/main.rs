// Module x Component
mod cpu_info;
mod memory_info; // RAM
mod disk_info;   // Disks and Partitions
mod gpu_info;

// Dependencies
use cpu_info::CpuInfo;
use memory_info::MemoryInfo;
use disk_info::DiskInfo;
use gpu_info::GpuInfo;
use sysinfo::{System, Disks};
use std::process::Command;

// Structure to store system information
#[derive(Debug)]
struct SystemInfo {
    cpu: CpuInfo,
    memory: MemoryInfo,
    disks: Vec<DiskInfo>,
}

impl SystemInfo {
    // Initialize system modules
    fn new(sys: &System, disks: &Disks) -> Self {
        Self {
            cpu: CpuInfo::get(sys),
            memory: MemoryInfo::get(sys),
            disks: DiskInfo::get(disks),
        }
    }

    // Print all system information
    fn display(&self) {
        println!("CPU: {} | {} Cores | {} MHz", self.cpu.model, self.cpu.cores, self.cpu.frequency);
        println!("RAM: {} MB used of {} MB", self.memory.used, self.memory.total);
        println!("Disks:");
        for disk in &self.disks {
            println!("  📂 {} - Total: {} GB | Free: {} GB", disk.name, disk.total_space, disk.available_space);
        }
        let gpu = GpuInfo::get();
        println!("GPU detected: {}, Driver: {}", gpu.name, gpu.driver);
    }
}

// Main function
fn main() {

    // Get execution arguments
    let args: Vec<String> = std::env::args().collect();

    // If executed without arguments, open terminal and run program with "display"
    if args.len() == 1 { 
        //run terminal on Windows.
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/K", "hrdwr_info.exe display"])
                .spawn()
                .expect("Error ejecutando cmd");
        //run terminal on Linux
        } else if cfg!(target_os = "linux") {
            Command::new("x-terminal-emulator")
                .args(&["-e", "bash", "-c", "./hrdwr_info display; exec bash"])
                .spawn()
                .expect("Error ejecutando terminal");
        } else {
            println!("Unknown OS");
        }
    }

    // If program is executed with "display", print system info and exit
    if args.contains(&String::from("display")) {
        let sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        let system_info = SystemInfo::new(&sys, &disks);
        system_info.display(); // Print system information
        return; // Exit after printing
    }
}
