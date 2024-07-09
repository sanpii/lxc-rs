fn main() {
    let c =
        lxc::Container::new("apicontainer", None).expect("Failed to setup lxc_container struct");

    if !c.is_defined() {
        c.create(
            Some("download"),
            None,
            None,
            lxc::CreateFlags::QUIET,
            &["-d", "ubuntu", "-r", "focal", "-a", "amd64"],
        )
        .expect("Failed to create container rootfs");
    }

    c.start(false, &[]).expect("Failed to start the container");

    println!("Container state: {}", c.state());
    println!("Container PID: {}", c.init_pid());
    println!("Interfaces: {:?}", c.get_interfaces());

    if c.shutdown(30).is_err() {
        println!("Failed to cleanly shutdown the container, forcing.");
        c.stop().expect("Failed to kill the container.");
    }

    c.destroy().expect("Failed to destroy the container.");
}
