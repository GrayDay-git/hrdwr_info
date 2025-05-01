//Module x Component.
mod cpu_info;
mod memory_info; //RAM
mod disk_info; //Disks and Partitions
mod gpu_info;

//Dependencies.
use cpu_info::CpuInfo;
use memory_info::MemoryInfo;
use disk_info::DiskInfo;
use gpu_info::GpuInfo;
use sysinfo::{System,Disks};

//receiving module's caracteristics.
#[derive(Debug)]
struct SystemInfo {
    cpu: CpuInfo,
    memory: MemoryInfo,
    disks: Vec<DiskInfo>
}

impl SystemInfo {
    //Initializing modules and characteristics.
    fn new(sys: &System, disks: &Disks) -> Self {
        Self {
            cpu: CpuInfo::get(sys),
            memory: MemoryInfo::get(sys),
            disks: DiskInfo::get(disks),
        }
    }

    //Print all the info.
    fn display(&self) 
    {
        println!("CPU: {} | {} Cores | {} MHz", self.cpu.model, self.cpu.cores, self.cpu.frequency); //Cpu printer.
        println!("RAM: {} MB used of {} MB",  self.memory.used, self.memory.total); //RAM printer.
        println!("Disks:");
        for disk in &self.disks
        {
            println!("  📂 {} - Total: {} GB | Free: {} GB", disk.name, disk.total_space, disk.available_space); //disk printer.
        }
        let gpu = GpuInfo::get(); //Get GPU info.
        println!("GPU detected: {}, Driver: {}", gpu.name, gpu.driver); // GPU printer.
    }
}

//Here is the begining.
fn main()
{
    //initialize the analized libraries.
    let sys = System::new_all();
    let disks= Disks::new_with_refreshed_list();
    let system_info = SystemInfo::new(&sys, &disks);

    system_info.display(); // Show hardware info.
}