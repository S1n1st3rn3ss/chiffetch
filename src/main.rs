use std::fs::{File, read_to_string};

fn main() {
    // Distribution name, ID and links
    let binding = read_to_string("/etc/os-release").unwrap();
    let s = binding.split("\n").next().unwrap();
    let cpu = read_to_string("/proc/cpuinfo").unwrap();
    println!("{}", s);
}
