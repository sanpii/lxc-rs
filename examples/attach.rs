use std::os::unix::io::AsRawFd;

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
        &["-d", "alpine", "-r", "3.14", "-a", "amd64"],
    )
    .expect("Failed to create container rootfs");

    c.start(false, &[]).expect("Failed to start the container");

    let mut options = lxc::attach::Options {
        stdout_fd: std::io::stdout().as_raw_fd(),
        stderr_fd: std::io::stderr().as_raw_fd(),
        stdin_fd: std::io::stdin().as_raw_fd(),

        ..lxc::attach::Options::default()
    };

    let r = c.attach(
        Some(lxc::attach::run_shell),
        std::ptr::null_mut() as *mut std::ffi::c_void,
        &mut options,
    );

    match r {
        Ok(pid) => wait(pid),
        Err(e) => eprintln!("{:?}", e),
    }

    c.stop().expect("Failed to kill the container.");
    c.destroy().expect("Failed to destroy the container.");

    Ok(())
}

fn wait(pid: i32) {
    let mut status = 0;

    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }
}
