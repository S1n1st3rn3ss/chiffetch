use chiffetch::*;

fn main() {
    let os = get_distro();
    let cpu = get_cpu();
    let temp = get_temp();
    let uptime = get_uptime();
    println!("{}h", uptime.unwrap().hours);
}

