use std::fs::{File, read_to_string};
use std::io;

fn main() {
    // Distribution name, ID and links
    let os = get_distro();
    let cpu = get_cpu();
    let temp = get_temp();
    // let s = binding.split("\n").next().unwrap();
    println!("{}", temp);
}

struct OS {
    name: String,
    version: String,
}
fn get_distro() -> String {
    let binding = read_to_string("/etc/os-release");
    match binding {
        Ok(string) => string,
        Err(error) => read_to_string("/usr/lib/os-release").unwrap()
    }
}
fn get_cpu() -> String {
    read_to_string("/proc/cpuinfo")
        .expect("cpuinfo is found")
}
fn get_temp() -> String {
    let temp = read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .expect("/thermal/thermal_zone*/temp exists")
        .trim_end()
        .to_owned();
    let mut float_temp = temp.parse::<f32>()
        .expect("/temp is parsed correctly");
    float_temp /= 1000.0;
    float_temp.to_string()
}