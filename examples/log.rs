fn main() -> lxc::Result {
    #[cfg(feature = "v2_0")]
    let log = lxc::Log {
        name: "demo".to_string(),
        lxcpath: lxc::get_global_config_item("lxc.lxcpath")?.unwrap_or_default(),
        file: "demo.log".to_string(),
        level: lxc::log::Level::Debug,
        prefix: "".to_string(),
        quiet: false,
    };

    #[cfg(feature = "v2_0")]
    log.init()?;

    let c = lxc::Container::new("apicontainer", None)?;

    c.create(
        "download",
        None,
        None,
        lxc::CreateFlags::QUIET,
        &["-d", "ubuntu", "-r", "trusty", "-a", "i386"],
    )?;

    c.start(false, &[])?;

    lxc::Log::close();

    Ok(())
}
