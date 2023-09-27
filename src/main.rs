use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io;

fn main() {
    let os = get_distro();
    let cpu = get_cpu();
    let temp = get_temp();
    println!("{}\n{}", os.name, temp);
}

struct OsInfo {
    name: String,
    // version: String,
}
fn get_distro() -> OsInfo {
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
fn get_kernel() -> String {
    let kernel = read_to_string("/proc/sys/kernel/osrelease")
        .expect("/proc/sys/kernel/osrelease was found");
    kernel
}
fn get_cpu() -> String {
    read_to_string("/proc/cpuinfo")
        .expect("cpuinfo is found")
}
fn get_temp() -> String {
    // thermal data might be different between distributions and/or kernel versions?
    let temp = read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .expect("/thermal/thermal_zone*/temp exists")
        .trim_end()
        .to_owned();
    let mut float_temp = temp.parse::<f32>()
        .expect("/temp is parsed correctly");
    float_temp /= 1000.0;
    let float_temp = float_temp.to_string() + "°C";
    float_temp
}