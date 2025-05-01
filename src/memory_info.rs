use sysinfo::System;

#[derive(Debug)]
pub struct MemoryInfo
{
    //characteristics to save.
    pub total: u64,
    pub used: u64,
}
impl MemoryInfo
{
    //get characteristics
    pub fn get(sys: &System) -> Self {
        Self {
            total: sys.total_memory() / 1024,  // Convert from KB to MB
            used: sys.used_memory() / 1024,    // Convert from KB to MB
        }
    }
}