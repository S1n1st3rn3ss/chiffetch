use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::error::Error;
use std::fs::{read_dir, read_to_string};
use std::io::ErrorKind;
use glob::*;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct OsInfo {
    pub name: String,
    version: String,
}
pub fn get_distro() -> OsInfo {
    let binding = read_to_string("/etc/lsb-release").expect("/etc/os-release was found");
    let binding: Vec<&str> = binding.lines().collect();
    let mut distro_info: HashMap<String, String> = Default::default();
    for i in binding {
        let split = i.split_once("=").unwrap();
        distro_info.insert(split.0.trim().to_owned(), split.1.trim().to_owned());
    }
    // Version field doesn't exist for Arch-based distributions
    let os_info = OsInfo {
        name: distro_info["DISTRIB_ID"]
            .replace("\"", "")
            .trim()
            .to_owned(),
        version: distro_info["DISTRIB_RELEASE"]
            .replace("\\", "")
            .replace("\"", "")
            .trim()
            .to_owned(),
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
pub struct Uptime {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
}
// checks uptime
pub fn get_uptime() -> Result<Uptime, Box<dyn Error>> {
    let uptime_path: &Path = Path::new("/proc/uptime");
    let uptime_bind = read_to_string(uptime_path)?;
    let uptime_total = uptime_bind
        .split_once(" ")
        .expect("uptime was split correctly");
    let uptime_system = uptime_total
        .0
        .to_owned()
        .parse::<f32>()?
        .round()
        as i32;
    let uptime = Uptime {
        days: &uptime_system / 60 / 60 / 24,
        hours: &uptime_system / 60 / 60 % 24,
        minutes: &uptime_system / 60 % 60,
        seconds: &uptime_system % 60
    };
    Ok(uptime)
}

pub fn get_temp() -> String {
    match get_thermal_zone() {
        Ok(str) => str,
        Err(error) => get_temp_monitor().unwrap()
    }
}
// checks thermal_zone temps
// has early returns on reading the path (doesn't exist on Arch?) and on string parse
fn get_thermal_zone() -> Result<String, Box<dyn Error>> {
    let thermal_path: PathBuf = Path::new("/sys/class/thermal/thermal_zone0/temp").to_path_buf();
    let temp_value: f32 = parse_temperature(thermal_path)?;
    Ok(format!("{:.1}°C", temp_value))
}
// checks hwmon temperatures
// TODO: add checks for various temp* files
fn get_temp_monitor() -> Result<String, Box<dyn Error>> {
    let mon_paths: Vec<_> = glob("/sys/class/hwmon/*")?
        .collect();
    for paths in mon_paths {
        let name_path: PathBuf = paths
            .as_ref()
            .unwrap()
            .join("name");
        let name_file: String = read_to_string(name_path)?;
        let name_file = name_file.trim();
        return match name_file.as_ref() {
            "cpu_thermal" |
            "coretemp" |
            "fam15h_power" |
            "k10temp" => {
                let temp_path = paths
                    .unwrap()
                    .join("temp1_input");
                let temp_float: f32 = parse_temperature(temp_path)?;
                Ok(format!("{:.1}°C", temp_float))
            }
            _ => continue,
        }
    }
    Err(Box::new(std::fmt::Error))
}
fn parse_temperature(path: PathBuf) -> Result<f32, Box<dyn Error>> {
    let temp: String = read_to_string(path)?;
    let temp = temp.trim();
    let temp: f32 = temp.parse::<f32>()? / 1000.0;
    return Ok(temp)
}
pub fn get_shell() -> Result<String, VarError> {
    // let shell_path = "a/b/c";
    // Ok(shell_path.trim_end_matches("/").to_owned())
    let shell = env::var("SHELL")?;
    // let shell_pathless = shell.strip
    Ok(shell)
}