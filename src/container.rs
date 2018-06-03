#[cfg(feature = "v1_1")]
use super::ffi::to_mut_cstr;
use super::ffi::{to_cstr, to_nullable_cstr};
use std::ptr::{null, null_mut};

macro_rules! call {
    ( $container:ident . $prop:ident ) => {{
        unsafe {
            (*$container.inner).$prop
        }
    }};

    ( $container:ident . $prop:ident -> c_str ) => {{
        let result = unsafe {
            (*$container.inner).$prop
        };

        let str = unsafe {
            ::std::ffi::CStr::from_ptr(result)
        };

        str.to_str()
            .unwrap()
            .to_string()
    }};

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> [c_str] ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result == null_mut() {
            Err(())
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
            Err(())
        }
    }};

    ( $container:ident . $method:ident( $( $arg:expr ),* ) -> int ) => {{
        let result = unsafe {
            (*$container.inner).$method.unwrap()($container.inner, $($arg,)*)
        };

        if result >= 0 {
            Ok(())
        } else {
            Err(())
        }
    }};
}

pub struct Container {
    inner: *mut ::lxc_sys::lxc_container,
}

impl Container {
    pub fn new(name: &str, config_path: Option<&::std::path::Path>) -> Result<Self, String> {
        let config_path = match config_path {
            Some(path) => to_cstr(path.to_str().unwrap()),
            None => null(),
        };

        let inner = unsafe { ::lxc_sys::lxc_container_new(to_cstr(name), config_path) };

        Ok(Self { inner })
    }

    pub fn get(&self) -> Result<(), ()> {
        let success = unsafe { ::lxc_sys::lxc_container_get(self.inner) };

        if success == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn error_string(&self) -> String {
        call!(self.error_string -> c_str)
    }

    pub fn error_num(&self) -> i32 {
        call!(self.error_num)
    }

    pub fn daemonize(&self) -> bool {
        call!(self.daemonize)
    }

    pub fn config_path(&self) -> String {
        call!(self.config_path -> c_str)
    }

    pub fn is_defined(&self) -> bool {
        call!(self.is_defined())
    }

    pub fn state(&self) -> String {
        call!(self.state() -> c_str)
    }

    pub fn is_running(&self) -> bool {
        call!(self.is_running())
    }

    pub fn freeze(&self) -> Result<(), ()> {
        call!(self.freeze() -> bool)
    }

    pub fn unfreeze(&self) -> Result<(), ()> {
        call!(self.freeze() -> bool)
    }

    pub fn init_pid(&self) -> i32 {
        call!(self.init_pid())
    }

    pub fn load_config(&self, alt_file: &str) -> Result<(), ()> {
        call!(self.load_config(to_cstr(alt_file)) -> bool)
    }

    pub fn start(&self, use_init: bool, argv: &[&str]) -> Result<(), ()> {
        let argv_ptr = if argv.is_empty() {
            null()
        } else {
            let mut argv: Vec<*const i8> = argv.iter().map(|e| to_cstr(*e)).collect();

            argv.push(null());

            argv.as_ptr()
        };

        call!(self.start(use_init as i32, argv_ptr) -> bool)
    }

    pub fn stop(&self) -> Result<(), ()> {
        call!(self.stop() -> bool)
    }

    pub fn want_daemonize(&self, state: bool) -> Result<(), ()> {
        call!(self.want_daemonize(state) -> bool)
    }

    pub fn want_close_all_fds(&self, state: bool) -> Result<(), ()> {
        call!(self.want_close_all_fds(state) -> bool)
    }

    pub fn config_file_name(&self) -> String {
        call!(self.config_file_name() -> c_str)
    }

    pub fn wait(&self, state: &str, timeout: i32) -> Result<(), ()> {
        call!(self.wait(to_cstr(state), timeout) -> bool)
    }

    pub fn set_config_item(&self, key: &str, value: &str) -> Result<(), ()> {
        call!(self.set_config_item(to_cstr(key), to_cstr(value)) -> bool)
    }

    pub fn destroy(&self) -> Result<(), ()> {
        call!(self.destroy() -> bool)
    }

    pub fn save_config(&self, alt_file: &str) -> Result<(), ()> {
        call!(self.save_config(to_cstr(alt_file)) -> bool)
    }

    pub fn create(
        &self,
        template: &str,
        bdevtype: Option<&str>,
        specs: Option<&::lxc_sys::bdev_specs>,
        flags: super::CreateFlags,
        argv: &[&str],
    ) -> Result<(), ()> {
        let specs = match specs {
            Some(specs) => unimplemented!(),
            None => null_mut(),
        };

        let mut argv: Vec<*const i8> = argv.iter().map(|e| to_cstr(*e)).collect();

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

    pub fn rename(&self, newname: &str) -> Result<(), ()> {
        call!(self.rename(to_cstr(newname)) -> bool)
    }

    pub fn reboot(&self) -> Result<(), ()> {
        call!(self.reboot() -> bool)
    }

    pub fn shutdown(&self, timeout: i32) -> Result<(), ()> {
        call!(self.shutdown(timeout) -> bool)
    }

    pub fn clear_config(&self) {
        call!(self.clear_config())
    }

    pub fn clear_config_item(&self, key: &str) -> Result<(), ()> {
        call!(self.clear_config_item(to_cstr(key)) -> bool)
    }

    pub fn get_config_item(&self, key: &str) -> String {
        let size = call!(self.get_config_item(to_cstr(key), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_config_item(to_cstr(key), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    pub fn get_running_config_item(&self, key: &str) -> String {
        call!(self.get_running_config_item(to_cstr(key)) -> c_str)
    }

    pub fn get_keys(&self, key: &str) -> String {
        let size = call!(self.get_keys(to_cstr(key), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_keys(to_cstr(key), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    pub fn get_interfaces(&self) -> Vec<String> {
        call!(self.get_interfaces() -> [c_str]).unwrap_or_default()
    }

    pub fn get_ips(&self, interfaces: &str, family: &str, scope: i32) -> Vec<String> {
        call!(self.get_ips(to_cstr(interfaces), to_cstr(family), scope) -> [c_str])
            .unwrap_or_default()
    }

    pub fn get_cgroup_item(&self, subsys: &str) -> String {
        let size = call!(self.get_cgroup_item(to_cstr(subsys), null_mut(), 0));
        let mut retv = Vec::with_capacity(size as usize);

        call!(self.get_cgroup_item(to_cstr(subsys), retv.as_mut_ptr() as *mut i8, size));

        String::from_utf8(retv).unwrap()
    }

    pub fn set_cgroup_item(&self, subsys: &str, value: &str) -> Result<(), ()> {
        call!(self.set_cgroup_item(to_cstr(subsys), to_cstr(value)) -> bool)
    }

    pub fn get_config_path(&self) -> String {
        call!(self.get_config_path() -> c_str)
    }

    pub fn set_config_path(&self, path: &str) -> Result<(), ()> {
        call!(self.set_config_path(to_cstr(path)) -> bool)
    }

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

    pub fn console_getfd(&self, ttynum: &mut i32, masterfd: &mut i32) -> Result<(), ()> {
        call!(self.console_getfd(ttynum, masterfd) -> int)
    }

    pub fn console(
        &self,
        ttynum: i32,
        stdinfd: i32,
        stdoutfd: i32,
        stderrfd: i32,
        escape: i32,
    ) -> Result<(), ()> {
        call!(self.console(ttynum, stdinfd, stdoutfd, stderrfd, escape) -> int)
    }

    pub fn attach(
        &self,
        exec_function: super::attach::ExecFn,
        exec_payload: &mut ::std::os::raw::c_void,
        options: &mut super::attach::Options,
    ) -> Result<i32, ()> {
        let mut attached_process = 0;

        let result =
            call!(self.attach(exec_function, exec_payload, options, &mut attached_process) -> int);

        match result {
            Ok(()) => Ok(attached_process),
            Err(()) => Err(()),
        }
    }

    pub fn attach_run_wait(
        &self,
        options: &mut super::attach::Options,
        program: &str,
        argv: Vec<&str>,
    ) -> Result<i32, ()> {
        let argv_ptr = if argv.is_empty() {
            null()
        } else {
            let mut argv: Vec<*const i8> = argv.iter().map(|e| to_cstr(*e)).collect();

            argv.push(null());

            argv.as_ptr()
        };

        let pid = call!(self.attach_run_wait(options, to_cstr(program), argv_ptr));

        if pid == -1 {
            Err(())
        } else {
            Ok(pid)
        }
    }

    pub fn snapshot(&self, commentfile: &str) -> Result<(), ()> {
        call!(self.snapshot(to_cstr(commentfile)) -> int)
    }

    pub fn snapshot_list(&self) -> Vec<super::Snapshot> {
        let mut list = Vec::new();
        call!(self.snapshot_list(&mut list.as_mut_ptr()));

        list
    }

    pub fn snapshot_restore(&self, snapname: &str, newname: &str) -> Result<(), ()> {
        call!(self.snapshot_restore(to_cstr(snapname), to_cstr(newname)) -> bool)
    }

    pub fn snapshot_destroy(&self, snapname: &str) -> Result<(), ()> {
        call!(self.snapshot_destroy(to_cstr(snapname)) -> bool)
    }

    pub fn may_control(&self) -> bool {
        call!(self.may_control())
    }

    pub fn add_device_node(&self, src_path: &str, dest_path: Option<&str>) -> Result<(), ()> {
        call!(self.add_device_node(to_cstr(src_path), to_nullable_cstr(dest_path)) -> bool)
    }

    pub fn remove_device_node(&self, src_path: &str, dest_path: Option<&str>) -> Result<(), ()> {
        call!(self.remove_device_node(to_cstr(src_path), to_nullable_cstr(dest_path)) -> bool)
    }

    #[cfg(feature = "v1_1")]
    pub fn attach_interface(&self, dev: &str, dst_dev: &str) -> Result<(), ()> {
        call!(self.attach_interface(to_cstr(dev), to_cstr(dst_dev)) -> bool)
    }

    #[cfg(feature = "v1_1")]
    pub fn detach_interface(&self, dev: &str, dst_dev: &str) -> Result<(), ()> {
        call!(self.detach_interface(to_cstr(dev), to_cstr(dst_dev)) -> bool)
    }

    #[cfg(feature = "v1_1")]
    pub fn checkpoint(&self, directory: &str, stop: bool, verbose: bool) -> Result<(), ()> {
        call!(self.checkpoint(to_mut_cstr(directory), stop, verbose) -> bool)
    }

    #[cfg(feature = "v1_1")]
    pub fn restore(&self, directory: &str, verbose: bool) -> Result<(), ()> {
        call!(self.restore(to_mut_cstr(directory), verbose) -> bool)
    }

    #[cfg(feature = "v1_1")]
    pub fn destroy_with_snapshots(&self) -> Result<(), ()> {
        call!(self.destroy_with_snapshots() -> bool)
    }

    #[cfg(feature = "v1_1")]
    pub fn snapshot_destroy_all(&self) -> Result<(), ()> {
        call!(self.snapshot_destroy_all() -> bool)
    }

    #[cfg(feature = "v2_0")]
    pub fn migrate(
        &self,
        cmd: u32,
        opts: &mut super::migrate::Opts,
        size: usize,
    ) -> Result<(), ()> {
        call!(self.migrate(cmd, opts, size as u32) -> int)
    }

    #[cfg(feature = "v3_0")]
    pub fn console_log(&self, log: &mut super::console::Log) -> Result<(), ()> {
        call!(self.console_log(log) -> int)
    }

    #[cfg(feature = "v3_0")]
    pub fn reboot2(&self, timetout: i32) -> Result<(), ()> {
        call!(self.reboot2(timetout) -> bool)
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            ::lxc_sys::lxc_container_put(self.inner);
        }
    }
}
