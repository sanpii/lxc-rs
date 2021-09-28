fn main() -> lxc::Result {
    println!("LXC version: {}", lxc::version()?);
    println!(
        "LXC path: {}",
        lxc::get_global_config_item("lxc.lxcpath")?.unwrap_or("?".to_string())
    );
    println!();

    println!("Wait states:");
    for state in lxc::wait_states()? {
        println!("- {}", state);
    }

    Ok(())
}
