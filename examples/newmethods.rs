extern crate lxc;

fn main() {
    println!("LXC version: {}", lxc::version());

    println!("LXC path = '{:?}'", lxc::get_lxc_path());

    let path = "/var/lib/lxc";
    println!(
        "All containers in path '{}': {:?}",
        path,
        lxc::list_all_containers(path)
    );
}
