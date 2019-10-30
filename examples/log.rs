fn main() {
    #[cfg(feature = "v2_0")]
    let log = lxc::Log {
        name: "demo".to_string(),
        lxcpath: lxc::get_global_config_item("lxc.lxcpath").unwrap(),
        file: "demo.log".to_string(),
        level: lxc::log::Level::Debug,
        prefix: "".to_string(),
        quiet: false,
    };

    #[cfg(feature = "v2_0")]
    log.init().expect("Unable to init log");

    let c =
        lxc::Container::new("apicontainer", None).expect("Failed to setup lxc_container struct");

    c.create(
        "download",
        None,
        None,
        lxc::CreateFlags::QUIET,
        &["-d", "ubuntu", "-r", "trusty", "-a", "i386"],
    ).expect("Failed to create container rootfs");

    c.start(false, &[]).expect("Failed to start the container");

    lxc::Log::close();
}
