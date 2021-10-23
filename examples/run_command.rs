fn main() -> std::io::Result<()> {
    let c =
        lxc::Container::new("apicontainer", None).expect("Failed to setup lxc_container struct");

    if c.is_defined() {
        panic!("Container already exists");
    }

    c.create(
        "download",
        None,
        None,
        ::lxc::CreateFlags::QUIET,
        &["-d", "centos", "-r", "7", "-a", "amd64"],
    )
    .expect("Failed to create container rootfs");

    c.start(false, &[]).expect("Failed to start the container");

    let prog = "/bin/ps";
    let args = [prog, "auxw"];
    let r = c.attach_run_wait(&mut lxc::attach::Options::default(), prog, &args);
    match r {
        Err(e) => println!("Error: {}", e),
        Ok(s) => println!("Ok, waitpid() status={}", s),
    }

    c.stop().expect("Failed to kill the container.");
    c.destroy().expect("Failed to destroy the container.");

    Ok(())
}
