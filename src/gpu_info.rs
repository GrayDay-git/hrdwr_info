use std::process::Command;

#[derive(Debug)]
pub struct GpuInfo {
    //characteristics to save.
    pub name: String,
    pub driver: String,
}

impl GpuInfo {
    //Get components.
    pub fn get() -> Self {
        //Get graphics components on Linux.
        #[cfg(target_os = "linux")]
        {
            let output = Command::new("sh")
                .arg("-c")
                .arg("lspci -nn | grep -i 'VGA'")
                .output()
                .expect("Fail executing lspci");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let gpu_name = output_str.lines().next().unwrap_or("Unknown GPU").to_string(); //get GPU name.

            Self {
                name: gpu_name,
                driver: "Driver detected".to_string(), //if detects drivers.
            }
        }

        //Get graphics components on Windows.
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("wmic")
                .args(&["path", "win32_videocontroller", "get", "name"])
                .output()
                .expect("Fail executing WMIC");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let gpu_name = output_str.lines().nth(1).unwrap_or("Unknown GPU").trim().to_string(); //get GPU name.

            Self {
                name: gpu_name,
                driver: "Driver detected".to_string(), //if detects drivers.
            }
        }
    }
}
