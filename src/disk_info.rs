use sysinfo::Disks;

#[derive(Debug)]
pub struct DiskInfo {
    //characteristics to save.
    pub name: String,
    pub available_space: u64, // free space to GB
    pub total_space: u64,     // total space to GB
}

impl DiskInfo {
    pub fn get(sys: &Disks) -> Vec<Self> {
        sys.list()
            .iter()
            .map(|disk| Self {
                name: disk.name().to_string_lossy().into_owned(), //get disk name.
                //get disk internal data:
                available_space: disk.available_space() / 1_073_741_824, // Convert to GB
                total_space: disk.total_space() / 1_073_741_824,         // Convert to GB
            })
            .collect() //saving data.
    }
}