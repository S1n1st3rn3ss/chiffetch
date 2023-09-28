use std::collections::HashMap;
use std::{env, fs};
use std::env::VarError;
use std::error::Error;
use std::fs::{File, read_to_string};
use glob::*;
use std::path::{Path, PathBuf};
use sysinfo;
use sysinfo::{System, SystemExt};

pub struct Info {
    pub distro: String,
    pub kernel_ver: String,
    pub host_name: String,

}
pub fn get_data() -> Info {
    let mut sys = System::new();
    sys.refresh_all();
    let info = Info {
        distro: sys.name().unwrap(),
        kernel_ver: sys.kernel_version().unwrap(),
        host_name: sys.host_name().unwrap(),
    };
    info
}
pub fn get_kernel() -> Result<String, Box<dyn Error>> {
    let kernel = read_to_string("/proc/sys/kernel/osrelease")?;
    Ok(kernel)
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
    let uptime_system = uptime_total.0;
    let uptime_float: i32 = uptime_system
        .parse::<f32>()?
        .round()
        as i32;
    let uptime = Uptime {
        days: &uptime_float / 60 / 60 / 24,
        hours: &uptime_float / 60 / 60 % 24,
        minutes: &uptime_float / 60 % 60,
        seconds: &uptime_float % 60
    };
    Ok(uptime)
}

pub fn get_terminal() -> Result<String, Box<dyn Error>> {
    let id = std::process::id();
    let unix_id = std::os::unix::process::parent_id();
    let path = format!("/proc/{}/status", id);
    let path = Path::new(&path);
    let file = read_to_string(path)?;
    let string = file.trim_start_matches("Name:")
        .trim()
        .lines()
        .next()
        .unwrap()
        .to_owned();

    Ok(string)

}
pub fn get_temp() -> String {
    match get_thermal_zone() {
        Ok(str) => str,
        Err(_error) => get_temp_monitor().unwrap()
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
pub struct Motherboard {
    pub manufacturer: String,
    pub model: String,
}
// gets motherboard manufacturer and model name
// TODO: Add other types of mobo info storage
// https://github.com/dylanaraps/neofetch/blob/ccd5d9f52609bbdcd5d8fa78c4fdb0f12954125f/neofetch#L1238
pub fn get_motherboard() -> Result<Motherboard, Box<dyn Error>> {
    if Path::exists("/sys/devices/virtual/dmi/id/board_vendor".as_ref()) {
        let manufacturer: String = read_to_string("/sys/devices/virtual/dmi/id/board_vendor")?.trim().to_owned();
        let model: String = read_to_string("/sys/devices/virtual/dmi/id/board_name")?.trim().to_owned();
        let motherboard = Motherboard {
            manufacturer,
            model
        };
        Ok(motherboard)
    } else {
        Err(Box::new(std::fmt::Error))
    }
}