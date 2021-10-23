fn main() -> lxc::Result {
    let c = lxc::Container::new("apicontainer", None)?;

    if !c.is_defined()? {
        c.create(
            "download",
            None,
            None,
            lxc::CreateFlags::QUIET,
            &["-d", "ubuntu", "-r", "focal", "-a", "amd64"],
        )?;
    }

    c.start(false, &[])?;

    println!("Container state: {}", c.state()?);
    println!("Container PID: {}", c.init_pid()?);
    println!("Interfaces: {:?}", c.get_interfaces()?);

    if c.shutdown(30).is_err() {
        println!("Failed to cleanly shutdown the container, forcing.");
        c.stop()?;
    }

    c.destroy()
}
