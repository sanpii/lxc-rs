use super::ffi::{release, to_cstr, to_nullable_cstr};
use std::ptr::null_mut;

macro_rules! get {
    ( $container:ident . $prop:ident ) => {{
        unsafe { (*$container.inner).$prop }
    }};

    ( $container:ident . $prop:ident -> c_str ) => {{
        let result = get!($container.$prop);

        let str = if result.is_null() {
            ""
        } else {
            unsafe { std::ffi::CStr::from_ptr(result).to_str().unwrap() }
        };

        str.to_string()
    }};
}

macro_rules! call {
    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> [c_str] ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result.is_null() {
            Err($container.last_error())
        } else {
            let vec = super::ffi::vec_from_nta(result);

            let vec = vec.iter()
                .map(|e| {
                    let str = unsafe {
                        std::ffi::CStr::from_ptr(*e)
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
            std::ffi::CStr::from_ptr(result)
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
    inner: *mut lxc_sys::lxc_container,
}

impl Container {
    /**
     * Create a new container.
     */
    pub fn new(
        name: &str,
        config_path: Option<&std::path::Path>,
    ) -> std::result::Result<Self, String> {
        let config_path = match config_path {
            Some(path) => to_cstr(path.to_str().unwrap()),
            None => null_mut(),
        };

        let c_name = to_cstr(name);
        let inner = unsafe { lxc_sys::lxc_container_new(c_name, config_path) };
        release(c_name);
        release(config_path);

        Ok(Self { inner })
    }

    /**
     * Add a reference to the specified container.
     */
    pub fn get(&self) -> super::Result<()> {
        let success = unsafe { lxc_sys::lxc_container_get(self.inner) };

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
        let c_alt_file = to_cstr(alt_file);
        let r = call!(self.load_config(c_alt_file) -> bool);
        release(c_alt_file);
        r
    }

    /**
     * Start the container.
     */
    pub fn start(&self, use_init: bool, argv: &[&str]) -> super::Result<()> {
        if argv.is_empty() {
            call!(self.start(use_init as i32, null_mut()) -> bool)
        } else {
            let mut argv: Vec<*mut i8> = argv.iter().map(|e| to_cstr(*e)).collect();
            argv.push(null_mut());

            let r = call!(self.start(use_init as i32, argv.as_mut_ptr()) -> bool);
            let _ = argv.iter().map(|e| release(*e));
            r
        }
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
        let c_state = to_cstr(state);
        let r = call!(self.wait(c_state, timeout) -> bool);
        release(c_state);
        r
    }

    /**
     * Set a key/value configuration option.
     */
    pub fn set_config_item(&self, key: &str, value: &str) -> super::Result<()> {
        let c_key = to_cstr(key);
        let c_value = to_cstr(value);
        let r = call!(self.set_config_item(c_key, c_value) -> bool);
        release(c_key);
        release(c_value);
        r
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
        let c_alt_file = to_cstr(alt_file);
        let r = call!(self.save_config(c_alt_file) -> bool);
        release(c_alt_file);
        r
    }

    /**
     * Create a container.
     */
    pub fn create(
        &self,
        template: &str,
        bdevtype: Option<&str>,
        specs: Option<&mut lxc_sys::bdev_specs>,
        flags: super::CreateFlags,
        argv: &[&str],
    ) -> super::Result<()> {
        let specs = match specs {
            Some(specs) => &mut *specs,
            None => null_mut(),
        };

        let mut argv: Vec<*mut i8> = argv.iter().map(|e| to_cstr(*e)).collect();

        argv.push(null_mut());

        let c_template = to_cstr(template);
        let c_bdevtype = match bdevtype {
            Some(value) => to_cstr(value),
            None => null_mut(),
        };
        let r = call!(
            self.create(
                c_template,
                c_bdevtype,
                specs,
                flags.bits(),
                argv.as_ptr()
            ) -> bool
        );
        release(c_template);
        release(c_bdevtype);
        let _ = argv.iter().map(|e| release(*e));
        r
    }

    /**
     * Rename a container.
     */
    pub fn rename(&self, newname: &str) -> super::Result<()> {
        let c_newname = to_cstr(newname);
        let r = call!(self.rename(c_newname) -> bool);
        release(c_newname);
        r
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
        let c_key = to_cstr(key);
        let r = call!(self.clear_config_item(c_key) -> bool);
        release(c_key);
        r
    }

    /**
     * Retrieve the value of a config item.
     */
    pub fn get_config_item(&self, key: &str) -> Option<String> {
        let c_key = to_cstr(key);
        let size = call!(self.get_config_item(c_key, null_mut(), 0));
        if size < 0 {
            return None;
        }
        let mut retv = vec![0; size as usize];

        call!(self.get_config_item(c_key, retv.as_mut_ptr() as *mut i8, size + 1));
        release(c_key);

        Some(String::from_utf8(retv).unwrap())
    }

    /**
     * Retrieve the value of a config item from running container.
     */
    pub fn get_running_config_item(&self, key: &str) -> String {
        let c_key = to_cstr(key);
        let r = call!(self.get_running_config_item(c_key) -> c_str);
        release(c_key);
        r
    }

    /**
     * Retrieve a list of config item keys given a key prefix.
     */
    pub fn get_keys(&self, key: &str) -> String {
        let c_key = to_cstr(key);
        let size = call!(self.get_keys(c_key, null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_keys(c_key, retv.as_mut_ptr() as *mut i8, size));
        release(c_key);

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
        let c_interfaces = to_cstr(interfaces);
        let c_family = to_cstr(family);
        let r = call!(self.get_ips(c_interfaces, c_family, scope) -> [c_str]).unwrap_or_default();
        release(c_interfaces);
        release(c_family);
        r
    }

    /**
     * Retrieve the specified cgroup subsystem value for the container.
     */
    pub fn get_cgroup_item(&self, subsys: &str) -> String {
        let c_subsys = to_cstr(subsys);
        let size = call!(self.get_cgroup_item(c_subsys, null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_cgroup_item(c_subsys, retv.as_mut_ptr() as *mut i8, size));
        release(c_subsys);

        String::from_utf8(retv).unwrap()
    }

    /**
     * Set the specified cgroup subsystem value for the container.
     */
    pub fn set_cgroup_item(&self, subsys: &str, value: &str) -> super::Result<()> {
        let c_subsys = to_cstr(subsys);
        let r = call!(self.set_cgroup_item(c_subsys, to_cstr(value)) -> bool);
        release(c_subsys);
        r
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
        let c_path = to_cstr(path);
        let r = call!(self.set_config_path(c_path) -> bool);
        release(c_path);
        r
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
        hookargs: &[String],
    ) -> Self {
        if !hookargs.is_empty() {
            unimplemented!();
        }

        let c_newname = to_cstr(newname);
        let c_lxcpath = to_cstr(lxcpath);
        let c_bdevtype = to_cstr(bdevtype);
        let c_bdevdata = to_cstr(bdevdata);
        let inner = call!(self.clone(
            c_newname,
            c_lxcpath,
            flags,
            c_bdevtype,
            c_bdevdata,
            newsize,
            null_mut()
        ));
        release(c_newname);
        release(c_lxcpath);
        release(c_bdevtype);
        release(c_bdevdata);

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
        exec_payload: &mut std::os::raw::c_void,
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
        argv: &[&str],
    ) -> super::Result<i32> {
        let mut argv: Vec<*mut i8> = argv.iter().map(|e| to_cstr(*e)).collect();
        argv.push(null_mut());

        let c_program = to_cstr(program);
        let pid =
            call!(self.attach_run_wait(options, c_program, argv.as_ptr() as *const *const i8));
        let _ = argv.iter().map(|e| release(*e));
        release(c_program);

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
        let c_commentfile = to_cstr(commentfile);
        let r = call!(self.snapshot(c_commentfile) -> int);
        release(c_commentfile);
        r
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
        let c_snapname = to_cstr(snapname);
        let c_newname = to_cstr(newname);
        let r = call!(self.snapshot_restore(c_snapname, c_newname) -> bool);
        release(c_snapname);
        release(c_newname);
        r
    }

    /**
     * Destroy the specified snapshot.
     */
    pub fn snapshot_destroy(&self, snapname: &str) -> super::Result<()> {
        let c_snapname = to_cstr(snapname);
        let r = call!(self.snapshot_destroy(c_snapname) -> bool);
        release(c_snapname);
        r
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
        let c_src_path = to_cstr(src_path);
        let r = call!(self.add_device_node(c_src_path, to_nullable_cstr(dest_path)) -> bool);
        release(c_src_path);
        r
    }

    /**
     * Remove specified device from the container.
     */
    pub fn remove_device_node(&self, src_path: &str, dest_path: Option<&str>) -> super::Result<()> {
        let c_src_path = to_cstr(src_path);
        let r = call!(self.remove_device_node(c_src_path, to_nullable_cstr(dest_path)) -> bool);
        release(c_src_path);
        r
    }

    /**
     * Add specified netdev to the container.
     */
    #[cfg(feature = "v1_1")]
    pub fn attach_interface(&self, dev: &str, dst_dev: &str) -> super::Result<()> {
        let c_dev = to_cstr(dev);
        let c_dst_dev = to_cstr(dst_dev);
        let r = call!(self.attach_interface(c_dev, c_dst_dev) -> bool);
        release(c_dev);
        release(c_dst_dev);
        r
    }

    /**
     * Remove specified netdev from the container.
     */
    #[cfg(feature = "v1_1")]
    pub fn detach_interface(&self, dev: &str, dst_dev: &str) -> super::Result<()> {
        let c_dev = to_cstr(dev);
        let c_dst_dev = to_cstr(dst_dev);
        let r = call!(self.detach_interface(c_dev, c_dst_dev) -> bool);
        release(c_dev);
        release(c_dst_dev);
        r
    }

    /**
     * Checkpoint a container.
     */
    #[cfg(feature = "v1_1")]
    pub fn checkpoint(&self, directory: &str, stop: bool, verbose: bool) -> super::Result<()> {
        let c_directory = to_cstr(directory);
        let r = call!(self.checkpoint(c_directory, stop, verbose) -> bool);
        release(c_directory);
        r
    }

    /**
     * Restore a container from a checkpoint.
     */
    #[cfg(feature = "v1_1")]
    pub fn restore(&self, directory: &str, verbose: bool) -> super::Result<()> {
        let c_directory = to_cstr(directory);
        let r = call!(self.restore(c_directory, verbose) -> bool);
        release(c_directory);
        r
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

    fn last_error(&self) -> super::Error {
        super::Error {
            num: get!(self.error_num),
            str: get!(self.error_string -> c_str),
        }
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            lxc_sys::lxc_container_put(self.inner);
        }
    }
}
