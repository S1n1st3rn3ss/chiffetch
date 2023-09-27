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
fn get_thermal_zone() -> Result<String, String> {
    let temp = read_to_string("/sys/class/thermal/thermal_zone0/temp");
    let temp_value = match temp {
        Ok(str) => str.trim().to_owned(),
        Err(e) => return Err(e.to_string()),
    };
    match temp_value.parse::<f32>() {
        Ok(float) => Ok((float / 1000.0).to_string()),
        Err(e) => return Err(e.to_string()),
    }
}
fn get_temp_monitor() -> Result<String, String> {
    let mon_paths: Vec<_> = glob("/sys/class/hwmon/*/name")
        .expect("paths found")
        .collect();
    let thing: Vec<_> = glob("/sys/class/hwmon/*").unwrap().collect();
    for paths in thing {
        let mut new_path = paths.unwrap();
        new_path.push("name");
        println!("{}", read_to_string(new_path).unwrap().trim());
    }
    Ok("balls".to_string())
}