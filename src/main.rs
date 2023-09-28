use chiffetch::*;

fn main() {
    let os = get_data();
    let cpu = get_cpu();
    let temp = get_temp();
    let uptime = get_uptime();
    let mobo = get_motherboard();
    let term = get_terminal();
    println!("{}\n{}\n{}", os.host_name, os.kernel_ver, os.distro);
}

