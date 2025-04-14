fn main() -> lxc::Result {
    println!("LXC version: {}", lxc::version());
    println!("LXC path: {}", lxc::path().unwrap_or("?".to_string()),);
    println!();

    let path = "/var/lib/lxc";
    println!("All containers in path '{path}':");
    for container in lxc::list_all_containers(path)? {
        println!("- {container}");
    }
    println!();

    println!("Wait states:");
    for state in lxc::wait_states() {
        println!("- {state}");
    }

    Ok(())
}
