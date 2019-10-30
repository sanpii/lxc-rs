extern crate lxc;

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
        &["-d", "centos", "-r", "7", "-a", "amd64"],
    )
    .expect("Failed to create container rootfs");

    c.start(false, &[]).expect("Failed to start the container");

    let mut options = lxc::attach::Options {
        attach_flags: 0,
        env_policy: 0,
        extra_env_vars: std::ptr::null_mut(),
        gid: 0,
        uid: 0,
        extra_keep_env: std::ptr::null_mut(),
        initial_cwd: std::ptr::null_mut(),
        #[cfg(feature = "v3_0")]
        log_fd: std::io::stdout().as_raw_fd(),
        stdout_fd: std::io::stdout().as_raw_fd(),
        stderr_fd: std::io::stderr().as_raw_fd(),
        stdin_fd: std::io::stdin().as_raw_fd(),
        namespaces: -1,
        personality: -1,
    };
    let prog = "/bin/ps";
    let args = [prog, "auxw"];
    let r = c.attach_run_wait(&mut options, prog, &args);
    match r {
        Err(e) => println!("Error: {}", e),
        Ok(s) => println!("Ok, waitpid() status={}", s),
    }

    c.stop().expect("Failed to kill the container.");
    c.destroy().expect("Failed to destroy the container.");

    Ok(())
}
