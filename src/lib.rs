use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_dir, read_to_string};
use glob::*;
use std::path::Path;

pub struct OsInfo {
    pub name: String,
    // version: String,
}
pub fn get_distro() -> OsInfo {
    let binding = read_to_string("/etc/os-release").expect("/etc/os-release was found");
    let binding: Vec<&str> = binding.lines().collect();
    let mut distro_info: HashMap<String, String> = Default::default();
    for i in binding {
        let split = i.split_once("=").unwrap();
        distro_info.insert(split.0.trim().to_owned(), split.1.trim().to_owned());
    }
    // Version field doesn't exist for Arch-based distributions
    let os_info = OsInfo {
        name: distro_info["NAME"]
            .replace("\"", "")
            .trim()
            .to_owned(),
        // version: distro_info["VERSION"]
        //     .replace("\\", "")
        //     .replace("\"", "")
        //     .trim()
        //     .to_owned(),
    };
    os_info
}
pub fn get_kernel() -> String {
    let kernel = read_to_string("/proc/sys/kernel/osrelease")
        .expect("/proc/sys/kernel/osrelease was found");
    kernel
}
pub fn get_cpu_frequency() -> String {
    read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/bios_limit")
        .expect("bios_limit of cpu frequency is found")
}
pub fn get_cpu() -> String {
    read_to_string("/proc/cpuinfo")
        .expect("cpuinfo is found")
}
// pub fn get_temp() -> String {
//     // thermal data might be different between distributions and/or kernel versions?

// }
pub fn get_temp() -> String {
    match get_thermal_zone() {
        Ok(str) => str,
        Err(error) => get_temp_monitor().unwrap()
    }
}
// checks thermal_zone temps
// has early returns on reading the path (doesn't exist on Arch?) and on string parse
fn get_thermal_zone() -> Result<String, Box<dyn std::error::Error>> {
    let thermal_path: &Path = Path::new("/sys/class/thermal/thermal_zone0/temp");
    let temp_value: f32 = read_to_string(thermal_path)?
        .trim()
        .to_owned()
        .parse()?;
    let temp_human = temp_value / 1000.0;
    Ok(format!("{:.2}°C", temp_human))
}
// checks hwmon temperatures
// TODO: make proper iteration over possible names
// TODO: add checks for various temp* files
fn get_temp_monitor() -> Result<String, String> {
    let possible_path = vec!["cpu_thermal",
                             "coretemp",
                             "fam15h_power",
                             "k10temp"];
    let mon_paths: Vec<_> = glob("/sys/class/hwmon/*")
        .expect("paths found")
        .collect();
    for paths in mon_paths {
        let name_path = paths
            .as_ref()
            .unwrap()
            .join("name");
        let name_path = read_to_string(name_path)
            .unwrap()
            .trim()
            .to_owned();
        if name_path == possible_path[3] {
            let temp_path = paths
                .unwrap()
                .join("temp1_input");
            let temp_string: f32 = read_to_string(temp_path)
                .unwrap()
                .trim()
                .parse::<f32>()?
                .unwrap()
                / 1000.0;
            return Ok(format!("{:.2}°C", temp_string));
        }
    }
    Err("???".to_owned())
}