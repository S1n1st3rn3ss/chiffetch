use chiffetch::*;

fn main() {
    let os = get_distro();
    let cpu = get_cpu();
    let temp = get_temp();
    let uptime = get_uptime();
    println!("{}\n{}\n{}\n", os.name, uptime.unwrap().minutes, temp);
}

