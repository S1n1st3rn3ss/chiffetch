use chiffetch::*;

fn main() {
    let os = get_distro();
    let cpu = get_cpu();
    let temp = get_temp();
    println!("{}\n{}", os.name, temp);
}

