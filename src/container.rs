use crate::ffi::to_cstr;
#[cfg(feature = "v1_1")]
use crate::ffi::to_mut_cstr;
use std::ptr::{null, null_mut};

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
            let vec = crate::ffi::vec_from_nta(result);

            let vec = vec.iter()
                .map(|e| {
                    let str = unsafe {
                        std::ffi::CStr::from_ptr(*e)
                    };

                    str.to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<_>>();

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
            Some(path) => cstr!(path.to_str().unwrap()),
            None => null(),
        };

        let inner = unsafe { lxc_sys::lxc_container_new(cstr!(name), config_path) };

        if inner.is_null() {
            Err(format!("Unable to create container {name}"))
        } else {
            Ok(Self { inner })
        }
    }

    /**
     * Add a reference to the specified container.
     */
    pub fn get(&self) -> crate::Result {
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
    #[must_use]
    pub fn error_string(&self) -> String {
        get!(self.error_string -> c_str)
    }

    /**
     * Last error number.
     */
    #[must_use]
    pub fn error_num(&self) -> i32 {
        get!(self.error_num)
    }

    /**
     * Whether container wishes to be daemonized.
     */
    #[must_use]
    pub fn daemonize(&self) -> bool {
        get!(self.daemonize)
    }

    /**
     * Full path to configuration file.
     */
    #[must_use]
    pub fn config_path(&self) -> String {
        get!(self.config_path -> c_str)
    }

    /**
     * Determine if `/var/lib/lxc/$name/config` exists.
     */
    #[must_use]
    pub fn is_defined(&self) -> bool {
        call!(self.is_defined())
    }

    /**
     * Wait for container to reach a particular state.
     */
    #[must_use]
    pub fn state(&self) -> String {
        call!(self.state() -> c_str)
    }

    /**
     * Determine if container is running.
     */
    #[must_use]
    pub fn is_running(&self) -> bool {
        call!(self.is_running())
    }

    /**
     * Freeze running container.
     */
    pub fn freeze(&self) -> crate::Result {
        call!(self.freeze() -> bool)
    }

    /**
     * Thaw a frozen container.
     */
    pub fn unfreeze(&self) -> crate::Result {
        call!(self.freeze() -> bool)
    }

    /**
     * Determine process ID of the containers init process.
     */
    #[must_use]
    pub fn init_pid(&self) -> i32 {
        call!(self.init_pid())
    }

    /**
     * Load the specified configuration for the container.
     */
    pub fn load_config(&self, alt_file: &str) -> crate::Result {
        call!(self.load_config(cstr!(alt_file)) -> bool)
    }

    /**
     * Start the container.
     */
    pub fn start(&self, use_init: bool, argv: &[&str]) -> crate::Result {
        let mut argv: Vec<*mut i8> = argv.iter().map(|e| to_cstr(e).into_raw()).collect();
        argv.push(null_mut());

        call!(self.start(use_init as i32, argv.as_mut_ptr()) -> bool)
    }

    /**
     * Stop the container.
     */
    pub fn stop(&self) -> crate::Result {
        call!(self.stop() -> bool)
    }

    /**
     * Change whether the container wants to run disconnected from the terminal.
     */
    pub fn want_daemonize(&self, state: bool) -> crate::Result {
        call!(self.want_daemonize(state) -> bool)
    }

    /**
     * Change whether the container wishes all file descriptors to be closed on
     *  to be closed on startup. The LISTEN_FDS environment variable
     *  can be set to keep inherited file descriptors open.
     */
    pub fn want_close_all_fds(&self, state: bool) -> crate::Result {
        call!(self.want_close_all_fds(state) -> bool)
    }

    /**
     * Return current config file name.
     */
    #[must_use]
    pub fn config_file_name(&self) -> String {
        call!(self.config_file_name() -> c_str)
    }

    /**
     * Wait for container to reach a particular state.
     */
    pub fn wait(&self, state: &str, timeout: i32) -> crate::Result {
        call!(self.wait(cstr!(state), timeout) -> bool)
    }

    /**
     * Set a key/value configuration option.
     */
    pub fn set_config_item(&self, key: &str, value: &str) -> crate::Result {
        call!(self.set_config_item(cstr!(key), cstr!(value)) -> bool)
    }

    /**
     * Delete the container.
     */
    pub fn destroy(&self) -> crate::Result {
        call!(self.destroy() -> bool)
    }

    /**
     * Save configuaration to a file.
     */
    pub fn save_config(&self, alt_file: &str) -> crate::Result {
        call!(self.save_config(cstr!(alt_file)) -> bool)
    }

    /**
     * Create a container.
     */
    pub fn create(
        &self,
        template: Option<&str>,
        bdevtype: Option<&str>,
        specs: Option<&mut lxc_sys::bdev_specs>,
        flags: crate::CreateFlags,
        argv: &[&str],
    ) -> crate::Result {
        let specs = match specs {
            Some(specs) => &mut *specs,
            None => null_mut(),
        };

        let mut argv: Vec<*mut i8> = argv.iter().map(|e| to_cstr(e).into_raw()).collect();
        argv.push(null_mut());

        call!(
            self.create(
                match template {
                    Some(value) => cstr!(value),
                    None => null(),
                },
                match bdevtype {
                    Some(value) => cstr!(value),
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
    pub fn rename(&self, newname: &str) -> crate::Result {
        call!(self.rename(cstr!(newname)) -> bool)
    }

    /**
     * Request the container reboot by sending it `SIGINT`.
     */
    pub fn reboot(&self) -> crate::Result {
        call!(self.reboot() -> bool)
    }

    /**
     * Request the container shutdown by sending it `SIGPWR`.
     */
    pub fn shutdown(&self, timeout: i32) -> crate::Result {
        call!(self.shutdown(timeout) -> bool)
    }

    /**
     * Completely clear the containers in-memory configuration.
     */
    pub fn clear_config(&self) {
        call!(self.clear_config());
    }

    /**
     * Clear a configuration item.
     */
    pub fn clear_config_item(&self, key: &str) -> crate::Result {
        call!(self.clear_config_item(cstr!(key)) -> bool)
    }

    /**
     * Retrieve the value of a config item.
     */
    #[must_use]
    pub fn get_config_item(&self, key: &str) -> Option<String> {
        let size = call!(self.get_config_item(cstr!(key), null_mut(), 0));
        if size < 0 {
            return None;
        }
        let mut retv = vec![0; size as usize];

        call!(self.get_config_item(cstr!(key), retv.as_mut_ptr() as *mut i8, size + 1));

        Some(String::from_utf8(retv).unwrap())
    }

    /**
     * Retrieve the value of a config item from running container.
     */
    #[must_use]
    pub fn get_running_config_item(&self, key: &str) -> String {
        call!(self.get_running_config_item(cstr!(key)) -> c_str)
    }

    /**
     * Retrieve a list of config item keys given a key prefix.
     */
    #[must_use]
    pub fn get_keys(&self, key: &str) -> String {
        let size = call!(self.get_keys(cstr!(key), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_keys(cstr!(key), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    /**
     * Obtain a list of network interfaces.
     */
    #[must_use]
    pub fn get_interfaces(&self) -> Vec<String> {
        call!(self.get_interfaces() -> [c_str]).unwrap_or_default()
    }

    /**
     * Determine the list of container IP addresses.
     */
    #[must_use]
    pub fn get_ips(
        &self,
        interface: Option<&str>,
        family: Option<&str>,
        scope: std::os::raw::c_int,
    ) -> Vec<std::net::IpAddr> {
        call!(self.get_ips(interface.map_or(null(), |x| cstr!(x)), family.map_or(null(), |x| cstr!(x)), scope) -> [c_str])
            .unwrap_or_default()
            .iter()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    /**
     * Retrieve the specified cgroup subsystem value for the container.
     */
    #[must_use]
    pub fn get_cgroup_item(&self, subsys: &str) -> String {
        let size = call!(self.get_cgroup_item(cstr!(subsys), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_cgroup_item(cstr!(subsys), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    /**
     * Set the specified cgroup subsystem value for the container.
     */
    pub fn set_cgroup_item(&self, subsys: &str, value: &str) -> crate::Result {
        call!(self.set_cgroup_item(cstr!(subsys), cstr!(value)) -> bool)
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
    #[must_use]
    pub fn get_config_path(&self) -> String {
        call!(self.get_config_path() -> c_str)
    }

    /**
     * Set the full path to the containers configuration file.
     */
    pub fn set_config_path(&self, path: &str) -> crate::Result {
        call!(self.set_config_path(cstr!(path)) -> bool)
    }

    /**
     * Copy a stopped container.
     */
    #[must_use]
    #[allow(clippy::too_many_arguments)]
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

        let inner = call!(self.clone(
            cstr!(newname),
            cstr!(lxcpath),
            flags,
            cstr!(bdevtype),
            cstr!(bdevdata),
            newsize,
            null_mut()
        ));

        Self { inner }
    }

    /**
     * Allocate a console tty for the container.
     */
    pub fn console_getfd(&self, ttynum: &mut i32, ptxfd: &mut i32) -> crate::Result {
        call!(self.console_getfd(ttynum, ptxfd) -> int)
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
    ) -> crate::Result {
        call!(self.console(ttynum, stdinfd, stdoutfd, stderrfd, escape) -> int)
    }

    /**
     * Create a sub-process attached to a container and run a function inside it.
     */
    pub fn attach(
        &self,
        exec_function: crate::attach::ExecFn,
        exec_payload: &mut std::os::raw::c_void,
        options: &mut crate::attach::Options,
    ) -> crate::Result<i32> {
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
        options: &mut crate::attach::Options,
        program: &str,
        argv: &[&str],
    ) -> crate::Result<i32> {
        let mut argv: Vec<*const i8> = argv.iter().map(|e| cstr!(*e)).collect();
        argv.push(null());

        let pid = call!(self.attach_run_wait(options, cstr!(program), argv.as_ptr()));

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
    pub fn snapshot(&self, commentfile: &str) -> crate::Result {
        call!(self.snapshot(cstr!(commentfile)) -> int)
    }

    /**
     * Obtain a list of container snapshots.
     */
    #[must_use]
    pub fn snapshot_list(&self) -> Vec<crate::Snapshot> {
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
    pub fn snapshot_restore(&self, snapname: &str, newname: &str) -> crate::Result {
        call!(self.snapshot_restore(cstr!(snapname), cstr!(newname)) -> bool)
    }

    /**
     * Destroy the specified snapshot.
     */
    pub fn snapshot_destroy(&self, snapname: &str) -> crate::Result {
        call!(self.snapshot_destroy(cstr!(snapname)) -> bool)
    }

    /**
     * Determine if the caller may control the container.
     */
    #[must_use]
    pub fn may_control(&self) -> bool {
        call!(self.may_control())
    }

    /**
     * Add specified device to the container.
     */
    pub fn add_device_node(&self, src_path: &str, dest_path: Option<&str>) -> crate::Result {
        let ptr = match dest_path {
            Some(s) => cstr!(s),
            None => std::ptr::null(),
        };

        call!(self.add_device_node(cstr!(src_path), ptr) -> bool)
    }

    /**
     * Remove specified device from the container.
     */
    pub fn remove_device_node(&self, src_path: &str, dest_path: Option<&str>) -> crate::Result {
        let ptr = match dest_path {
            Some(s) => cstr!(s),
            None => std::ptr::null(),
        };

        call!(self.remove_device_node(cstr!(src_path), ptr) -> bool)
    }

    /**
     * Add specified netdev to the container.
     */
    #[cfg(feature = "v1_1")]
    pub fn attach_interface(&self, dev: &str, dst_dev: &str) -> crate::Result {
        call!(self.attach_interface(cstr!(dev), cstr!(dst_dev)) -> bool)
    }

    /**
     * Remove specified netdev from the container.
     */
    #[cfg(feature = "v1_1")]
    pub fn detach_interface(&self, dev: &str, dst_dev: &str) -> crate::Result {
        call!(self.detach_interface(cstr!(dev), cstr!(dst_dev)) -> bool)
    }

    /**
     * Checkpoint a container.
     */
    #[cfg(feature = "v1_1")]
    pub fn checkpoint(&self, directory: &str, stop: bool, verbose: bool) -> crate::Result {
        call!(self.checkpoint(to_mut_cstr(directory).as_mut_ptr(), stop, verbose) -> bool)
    }

    /**
     * Restore a container from a checkpoint.
     */
    #[cfg(feature = "v1_1")]
    pub fn restore(&self, directory: &str, verbose: bool) -> crate::Result {
        call!(self.restore(to_mut_cstr(directory).as_mut_ptr(), verbose) -> bool)
    }

    /**
     * Delete the container and all its snapshots.
     */
    #[cfg(feature = "v1_1")]
    pub fn destroy_with_snapshots(&self) -> crate::Result {
        call!(self.destroy_with_snapshots() -> bool)
    }

    /**
     * Destroy all the container's snapshot.
     */
    #[cfg(feature = "v1_1")]
    pub fn snapshot_destroy_all(&self) -> crate::Result {
        call!(self.snapshot_destroy_all() -> bool)
    }

    /**
     * An API call to perform various migration operations.
     */
    #[cfg(feature = "v2_0")]
    pub fn migrate(&self, cmd: u32, opts: &mut crate::migrate::Opts, size: usize) -> crate::Result {
        call!(self.migrate(cmd, opts, size as u32) -> int)
    }

    /**
     * Query the console log of a container.
     */
    #[cfg(feature = "v3_0")]
    pub fn console_log(&self, log: &mut crate::console::Log) -> crate::Result {
        call!(self.console_log(log) -> int)
    }

    /**
     * Request the container reboot by sending it `SIGINT`.
     */
    #[cfg(feature = "v3_0")]
    pub fn reboot2(&self, timetout: i32) -> crate::Result {
        call!(self.reboot2(timetout) -> bool)
    }

    /**
     * Mount the host's path `source` onto the container's path `target`.
     */
    #[cfg(feature = "v3_1")]
    pub fn mount(
        &self,
        source: &str,
        target: &str,
        filesystemtype: &str,
        mountflags: u64,
        data: &std::os::raw::c_void,
        mnt: &mut crate::Mount,
    ) -> crate::Result {
        call!(self.mount(cstr!(source), cstr!(target), cstr!(filesystemtype), mountflags, data, mnt) -> int)
    }

    /**
     * Unmount the container's path `target`.
     */
    #[cfg(feature = "v3_1")]
    pub fn umount(&self, target: &str, mountflags: u64, mnt: &mut crate::Mount) -> crate::Result {
        call!(self.umount(cstr!(target), mountflags, mnt) -> int)
    }

    /**
     * Retrieve a file descriptor for the container's seccomp filter.
     */
    #[cfg(feature = "v3_2")]
    pub fn seccomp_notify_fd(&self) -> i32 {
        call!(self.seccomp_notify_fd())
    }

    /**
     * Retrieve a pidfd for the container's init process.
     */
    #[cfg(feature = "v4_0")]
    pub fn init_pidfd(&self) -> i32 {
        call!(self.init_pidfd())
    }

    /**
     * Retrieve a file descriptor for the running container's seccomp filter.
     */
    #[cfg(feature = "v5_0")]
    pub fn seccomp_notify_fd_active(&self) -> i32 {
        call!(self.seccomp_notify_fd_active())
    }

    /**
     * Retrieve a mount fd for the container's devpts instance.
     */
    #[cfg(feature = "v5_0")]
    pub fn devpts_fd(&self) -> i32 {
        call!(self.devpts_fd())
    }

    /**
     * Set response receive timeout for LXC commands.
     */
    #[cfg(feature = "v6_0")]
    pub fn set_timeout(&self, timeout: i32) -> crate::Result {
        call!(self.set_timeout(timeout) -> bool)
    }

    /**
     * Returns a raw pointer to the container.
     */
    #[must_use]
    pub fn as_ptr(&self) -> *const lxc_sys::lxc_container {
        self.inner
    }

    /**
     * Returns a mutable raw pointer to the container.
     */
    pub fn as_mut_ptr(&mut self) -> *mut lxc_sys::lxc_container {
        self.inner
    }

    fn last_error(&self) -> crate::Error {
        crate::Error {
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

impl std::fmt::Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! field_str {
            ($debug:ident, $inner:ident . $field:ident) => {
                if $inner.$field.is_null() {
                    $debug.field(stringify!($field), &Option::<&str>::None);
                } else {
                    $debug.field(stringify!($field), &crate::ffi::to_string($inner.$field));
                }
            };
        }

        macro_rules! field {
            ($debug:ident, $inner:ident . $field:ident) => {
                if $inner.$field.is_null() {
                    $debug.field(stringify!($field), &Option::<&str>::None);
                } else {
                    $debug.field(stringify!($field), &$inner.$field);
                }
            };
        }

        let mut debug = f.debug_struct("Container");

        if self.inner.is_null() {
            debug.field("inner", &"null");
        } else {
            let inner = unsafe { *self.inner };

            field_str!(debug, inner.name);
            field_str!(debug, inner.configfile);
            field_str!(debug, inner.pidfile);
            field!(debug, inner.slock);
            field!(debug, inner.privlock);
            debug.field("numthreads", &inner.numthreads);
            field!(debug, inner.lxc_conf);
            field_str!(debug, inner.error_string);
            debug.field("error_num", &inner.error_num);
            debug.field("daemonize", &inner.daemonize);
            field_str!(debug, inner.config_path);
        }

        debug.finish()
    }
}
