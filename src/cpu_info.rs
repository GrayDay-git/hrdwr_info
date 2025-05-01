use sysinfo::System;
use num_cpus;

#[derive(Debug)]
pub struct CpuInfo {
    //characteristics to save.
    pub model: String,
    pub cores: usize,
    pub frequency: u64,
}

impl CpuInfo {
    pub fn get(sys: &System) -> Self {
        let cpu = sys.cpus().get(0).unwrap_or_else(|| panic!("Unfounded CPU")); //Get cpu(s).
        let physicalcores = num_cpus::get_physical(); //Get quantity of cores.
        
        Self {
            model: cpu.brand().to_string(), //Get model name
            cores: physicalcores, //Give value to 'cores'
            frequency: cpu.frequency(), //Get cpu frecuency data.
        }
    }
}
