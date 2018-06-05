#[cfg(feature = "v1_1")]
use super::ffi::to_mut_cstr;
use super::ffi::{to_cstr, to_nullable_cstr};
use std::ptr::{null, null_mut};

macro_rules! get {
    ( $container:ident . $prop:ident ) => {{
        unsafe {
            (*$container.inner).$prop
        }
    }};

    ( $container:ident . $prop:ident -> c_str ) => {{
        let result = get!($container . $prop);

        let str = if result == null_mut() {
            ""
        } else {
            unsafe {
                ::std::ffi::CStr::from_ptr(result)
                    .to_str()
                    .unwrap()
            }
        };

        str.to_string()
    }};
}

macro_rules! call {
    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> [c_str] ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result == null_mut() {
            Err($container.last_error())
        } else {
            let slice = unsafe {
                unimplemented!();

                ::std::slice::from_raw_parts(result, 1)
            };

            let vec = slice.iter()
                .map(|e| {
                    let str = unsafe {
                        ::std::ffi::CStr::from_ptr(*e)
                    };

                    str.to_str()
                        .unwrap()
                        .to_string()
                })
                .collect();

            Ok(vec)
        }
    }};

    ( $container:ident . $method:ident( $( $arg:expr ),* ) ) => {
        unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        }
    };

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> c_str ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        let str = unsafe {
            ::std::ffi::CStr::from_ptr(result)
        };

        str.to_str()
            .unwrap()
            .to_string()
    }};

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> bool ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result {
            Ok(())
        } else {
            Err($container.last_error())
        }
    }};

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> int ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result >= 0 {
            Ok(())
        } else {
            Err($container.last_error())
        }
    }};
}

pub struct Container {
    inner: *mut ::lxc_sys::lxc_container,
}

impl Container {
    /**
     * Create a new container.
     */
    pub fn new(name: &str, config_path: Option<&::std::path::Path>) -> ::std::result::Result<Self, String> {
        let config_path = match config_path {
            Some(path) => to_cstr(path.to_str().unwrap()),
            None => null(),
        };

        let inner = unsafe {
            ::lxc_sys::lxc_container_new(to_cstr(name), config_path)
        };

        Ok(Self { inner })
    }

    /**
     * Add a reference to the specified container.
     */
    pub fn get(&self) -> super::Result<()> {
        let success = unsafe {
            ::lxc_sys::lxc_container_get(self.inner)
        };

        if success == 0 {
            Ok(())
        } else {
            Err(self.last_error())
        }
    }

    /**
     * Human-readable string representing last error.
     */
    pub fn error_string(&self) -> String {
        get!(self.error_string -> c_str)
    }

    /**
     * Last error number.
     */
    pub fn error_num(&self) -> i32 {
        get!(self.error_num)
    }

    /**
     * Whether container wishes to be daemonized.
     */
    pub fn daemonize(&self) -> bool {
        get!(self.daemonize)
    }

    /**
     * Full path to configuration file.
     */
    pub fn config_path(&self) -> String {
        get!(self.config_path -> c_str)
    }

    /**
     * Determine if `/var/lib/lxc/$name/config` exists.
     */
    pub fn is_defined(&self) -> bool {
        call!(self.is_defined())
    }

    /**
     * Wait for container to reach a particular state.
     */
    pub fn state(&self) -> String {
        call!(self.state() -> c_str)
    }

    /**
     * Determine if container is running.
     */
    pub fn is_running(&self) -> bool {
        call!(self.is_running())
    }

    /**
     * Freeze running container.
     */
    pub fn freeze(&self) -> super::Result<()> {
        call!(self.freeze() -> bool)
    }

    /**
     * Thaw a frozen container.
     */
    pub fn unfreeze(&self) -> super::Result<()> {
        call!(self.freeze() -> bool)
    }

    /**
     * Determine process ID of the containers init process.
     */
    pub fn init_pid(&self) -> i32 {
        call!(self.init_pid())
    }

    /**
     * Load the specified configuration for the container.
     */
    pub fn load_config(&self, alt_file: &str) -> super::Result<()> {
        call!(self.load_config(to_cstr(alt_file)) -> bool)
    }

    /**
     * Start the container.
     */
    pub fn start(&self, use_init: bool, argv: &[&str]) -> super::Result<()> {
        let argv_ptr = if argv.is_empty() {
            null()
        } else {
            let mut argv: Vec<*const i8> = argv.iter().map(|e| to_cstr(*e)).collect();

            argv.push(null());

            argv.as_ptr()
        };

        call!(self.start(use_init as i32, argv_ptr) -> bool)
    }

    /**
     * Stop the container.
     */
    pub fn stop(&self) -> super::Result<()> {
        call!(self.stop() -> bool)
    }

    /**
     * Change whether the container wants to run disconnected from the terminal.
     */
    pub fn want_daemonize(&self, state: bool) -> super::Result<()> {
        call!(self.want_daemonize(state) -> bool)
    }

    /**
     * Change whether the container wishes all file descriptors to be closed on
     * startup.
     */
    pub fn want_close_all_fds(&self, state: bool) -> super::Result<()> {
        call!(self.want_close_all_fds(state) -> bool)
    }

    /**
     * Return current config file name.
     */
    pub fn config_file_name(&self) -> String {
        call!(self.config_file_name() -> c_str)
    }

    /**
     * Wait for container to reach a particular state.
     */
    pub fn wait(&self, state: &str, timeout: i32) -> super::Result<()> {
        call!(self.wait(to_cstr(state), timeout) -> bool)
    }

    /**
     * Set a key/value configuration option.
     */
    pub fn set_config_item(&self, key: &str, value: &str) -> super::Result<()> {
        call!(self.set_config_item(to_cstr(key), to_cstr(value)) -> bool)
    }

    /**
     * Delete the container.
     */
    pub fn destroy(&self) -> super::Result<()> {
        call!(self.destroy() -> bool)
    }

    /**
     * Save configuaration to a file.
     */
    pub fn save_config(&self, alt_file: &str) -> super::Result<()> {
        call!(self.save_config(to_cstr(alt_file)) -> bool)
    }

    /**
     * Create a container.
     */
    pub fn create(
        &self,
        template: &str,
        bdevtype: Option<&str>,
        specs: Option<&::lxc_sys::bdev_specs>,
        flags: super::CreateFlags,
        argv: &[&str],
    ) -> super::Result<()> {
        let specs = match specs {
            Some(specs) => unimplemented!(),
            None => null_mut(),
        };

        let mut argv: Vec<*const i8> = argv.iter()
            .map(|e| to_cstr(*e))
            .collect();

        argv.push(null());

        call!(
            self.create(
                to_cstr(template),
                match bdevtype {
                    Some(value) => to_cstr(value),
                    None => null(),
                },
                specs,
                flags.bits(),
                argv.as_ptr()
            ) -> bool
        )
    }

    /**
     * Rename a container.
     */
    pub fn rename(&self, newname: &str) -> super::Result<()> {
        call!(self.rename(to_cstr(newname)) -> bool)
    }

    /**
     * Request the container reboot by sending it `SIGINT`.
     */
    pub fn reboot(&self) -> super::Result<()> {
        call!(self.reboot() -> bool)
    }

    /**
     * Request the container shutdown by sending it `SIGPWR`.
     */
    pub fn shutdown(&self, timeout: i32) -> super::Result<()> {
        call!(self.shutdown(timeout) -> bool)
    }

    /**
     * Completely clear the containers in-memory configuration.
     */
    pub fn clear_config(&self) {
        call!(self.clear_config())
    }

    /**
     * Clear a configuration item.
     */
    pub fn clear_config_item(&self, key: &str) -> super::Result<()> {
        call!(self.clear_config_item(to_cstr(key)) -> bool)
    }

    /**
     * Retrieve the value of a config item.
     */
    pub fn get_config_item(&self, key: &str) -> String {
        let size = call!(self.get_config_item(to_cstr(key), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_config_item(to_cstr(key), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    /**
     * Retrieve the value of a config item from running container.
     */
    pub fn get_running_config_item(&self, key: &str) -> String {
        call!(self.get_running_config_item(to_cstr(key)) -> c_str)
    }

    /**
     * Retrieve a list of config item keys given a key prefix.
     */
    pub fn get_keys(&self, key: &str) -> String {
        let size = call!(self.get_keys(to_cstr(key), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_keys(to_cstr(key), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    /**
     * Obtain a list of network interfaces.
     */
    pub fn get_interfaces(&self) -> Vec<String> {
        call!(self.get_interfaces() -> [c_str]).unwrap_or_default()
    }

    /**
     * Determine the list of container IP addresses.
     */
    pub fn get_ips(&self, interfaces: &str, family: &str, scope: i32) -> Vec<String> {
        call!(self.get_ips(to_cstr(interfaces), to_cstr(family), scope) -> [c_str])
            .unwrap_or_default()
    }

    /**
     * Retrieve the specified cgroup subsystem value for the container.
     */
    pub fn get_cgroup_item(&self, subsys: &str) -> String {
        let size = call!(self.get_cgroup_item(to_cstr(subsys), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_cgroup_item(to_cstr(subsys), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    /**
     * Set the specified cgroup subsystem value for the container.
     */
    pub fn set_cgroup_item(&self, subsys: &str, value: &str) -> super::Result<()> {
        call!(self.set_cgroup_item(to_cstr(subsys), to_cstr(value)) -> bool)
    }

    /**
     * Determine full path to the containers configuration file.
     *
     * Each container can have a custom configuration path. However by default
     * it will be set to either the `LXCPATH` configure variable, or the
     * `lxcpath` value in the `LXC_GLOBAL_CONF` configuration file
     * (i.e. `/etc/lxc/lxc.conf`). The value for a specific container can be
     * changed using `set_config_path`. There is no other way to specify this in
     * general at the moment.
     */
    pub fn get_config_path(&self) -> String {
        call!(self.get_config_path() -> c_str)
    }

    /**
     * Set the full path to the containers configuration file.
     */
    pub fn set_config_path(&self, path: &str) -> super::Result<()> {
        call!(self.set_config_path(to_cstr(path)) -> bool)
    }

    /**
     * Copy a stopped container.
     */
    pub fn clone(
        &self,
        newname: &str,
        lxcpath: &str,
        flags: i32,
        bdevtype: &str,
        bdevdata: &str,
        newsize: u64,
        hookargs: Vec<String>,
    ) -> Self {
        let inner = call!(self.clone(
            to_cstr(newname),
            to_cstr(lxcpath),
            flags,
            to_cstr(bdevtype),
            to_cstr(bdevdata),
            newsize,
            null_mut()
        ));

        Self { inner }
    }

    /**
     * Allocate a console tty for the container.
     */
    pub fn console_getfd(&self, ttynum: &mut i32, masterfd: &mut i32) -> super::Result<()> {
        call!(self.console_getfd(ttynum, masterfd) -> int)
    }

    /**
     * Allocate and run a console tty.
     */
    pub fn console(
        &self,
        ttynum: i32,
        stdinfd: i32,
        stdoutfd: i32,
        stderrfd: i32,
        escape: i32,
    ) -> super::Result<()> {
        call!(self.console(ttynum, stdinfd, stdoutfd, stderrfd, escape) -> int)
    }

    /**
     * Create a sub-process attached to a container and run a function inside it.
     */
    pub fn attach(
        &self,
        exec_function: super::attach::ExecFn,
        exec_payload: &mut ::std::os::raw::c_void,
        options: &mut super::attach::Options,
    ) -> super::Result<i32> {
        let mut attached_process = 0;

        let result =
            call!(self.attach(exec_function, exec_payload, options, &mut attached_process) -> int);

        match result {
            Ok(()) => Ok(attached_process),
            Err(err) => Err(err),
        }
    }

    /**
     * Run a program inside a container and wait for it to exit.
     */
    pub fn attach_run_wait(
        &self,
        options: &mut super::attach::Options,
        program: &str,
        argv: Vec<&str>,
    ) -> super::Result<i32> {
        let argv_ptr = if argv.is_empty() {
            null()
        } else {
            let mut argv: Vec<*const i8> = argv.iter().map(|e| to_cstr(*e)).collect();

            argv.push(null());

            argv.as_ptr()
        };

        let pid = call!(self.attach_run_wait(options, to_cstr(program), argv_ptr));

        if pid == -1 {
            Err(self.last_error())
        } else {
            Ok(pid)
        }
    }

    /**
     * Create a container snapshot.
     *
     * Assuming default paths, snapshots will be created as
     * `/var/lib/lxc/<c>/snaps/snap<n>` where `<c>` represents the container
     * name and `<n>` represents the zero-based snapshot number.
     */
    pub fn snapshot(&self, commentfile: &str) -> super::Result<()> {
        call!(self.snapshot(to_cstr(commentfile)) -> int)
    }

    /**
     * Obtain a list of container snapshots.
     */
    pub fn snapshot_list(&self) -> Vec<super::Snapshot> {
        let mut list = Vec::new();
        call!(self.snapshot_list(&mut list.as_mut_ptr()));

        list
    }

    /**
     * Create a new container based on a snapshot.
     *
     * The restored container will be a copy (not snapshot) of the snapshot,
     * and restored in the `lxcpath` of the original container.
     */
    pub fn snapshot_restore(&self, snapname: &str, newname: &str) -> super::Result<()> {
        call!(self.snapshot_restore(to_cstr(snapname), to_cstr(newname)) -> bool)
    }

    /**
     * Destroy the specified snapshot.
     */
    pub fn snapshot_destroy(&self, snapname: &str) -> super::Result<()> {
        call!(self.snapshot_destroy(to_cstr(snapname)) -> bool)
    }

    /**
     * Determine if the caller may control the container.
     */
    pub fn may_control(&self) -> bool {
        call!(self.may_control())
    }

    /**
     * Add specified device to the container.
     */
    pub fn add_device_node(&self, src_path: &str, dest_path: Option<&str>) -> super::Result<()> {
        call!(self.add_device_node(to_cstr(src_path), to_nullable_cstr(dest_path)) -> bool)
    }

    /**
     * Remove specified device from the container.
     */
    pub fn remove_device_node(&self, src_path: &str, dest_path: Option<&str>) -> super::Result<()> {
        call!(self.remove_device_node(to_cstr(src_path), to_nullable_cstr(dest_path)) -> bool)
    }

    /**
     * Add specified netdev to the container.
     */
    #[cfg(feature = "v1_1")]
    pub fn attach_interface(&self, dev: &str, dst_dev: &str) -> super::Result<()> {
        call!(self.attach_interface(to_cstr(dev), to_cstr(dst_dev)) -> bool)
    }

    /**
     * Remove specified netdev from the container.
     */
    #[cfg(feature = "v1_1")]
    pub fn detach_interface(&self, dev: &str, dst_dev: &str) -> super::Result<()> {
        call!(self.detach_interface(to_cstr(dev), to_cstr(dst_dev)) -> bool)
    }

    /**
     * Checkpoint a container.
     */
    #[cfg(feature = "v1_1")]
    pub fn checkpoint(&self, directory: &str, stop: bool, verbose: bool) -> super::Result<()> {
        call!(self.checkpoint(to_mut_cstr(directory), stop, verbose) -> bool)
    }

    /**
     * Restore a container from a checkpoint.
     */
    #[cfg(feature = "v1_1")]
    pub fn restore(&self, directory: &str, verbose: bool) -> super::Result<()> {
        call!(self.restore(to_mut_cstr(directory), verbose) -> bool)
    }

    /**
     * Delete the container and all its snapshots.
     */
    #[cfg(feature = "v1_1")]
    pub fn destroy_with_snapshots(&self) -> super::Result<()> {
        call!(self.destroy_with_snapshots() -> bool)
    }

    /**
     * Destroy all the container's snapshot.
     */
    #[cfg(feature = "v1_1")]
    pub fn snapshot_destroy_all(&self) -> super::Result<()> {
        call!(self.snapshot_destroy_all() -> bool)
    }

    /**
     * An API call to perform various migration operations.
     */
    #[cfg(feature = "v2_0")]
    pub fn migrate(
        &self,
        cmd: u32,
        opts: &mut super::migrate::Opts,
        size: usize,
    ) -> super::Result<()> {
        call!(self.migrate(cmd, opts, size as u32) -> int)
    }

    /**
     * Query the console log of a container.
     */
    #[cfg(feature = "v3_0")]
    pub fn console_log(&self, log: &mut super::console::Log) -> super::Result<()> {
        call!(self.console_log(log) -> int)
    }

    /**
     * Request the container reboot by sending it `SIGINT`.
     */
    #[cfg(feature = "v3_0")]
    pub fn reboot2(&self, timetout: i32) -> super::Result<()> {
        call!(self.reboot2(timetout) -> bool)
    }

    fn last_error(&self) -> super::Error
    {
        (
            get!(self.error_num),
            get!(self.error_string -> c_str),
        )
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            ::lxc_sys::lxc_container_put(self.inner);
        }
    }
}
