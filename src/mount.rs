#[derive(Clone)]
pub struct Mount {
    inner: *mut lxc_sys::lxc_mount,
}

impl Mount {
    pub fn version(&self) -> std::os::raw::c_int {
        prop!(self.version)
    }

    /**
     * Returns a raw pointer to the snapshot.
     */
    pub fn as_ptr(&self) -> *const lxc_sys::lxc_mount {
        self.inner
    }

    /**
     * Returns a mutable raw pointer to the snapshot.
     */
    pub fn as_mut_ptr(&mut self) -> *mut lxc_sys::lxc_mount {
        self.inner
    }
}

#[doc(hidden)]
impl From<&*mut lxc_sys::lxc_mount> for Mount {
    fn from(inner: &*mut lxc_sys::lxc_mount) -> Self {
        Self { inner: *inner }
    }
}

impl std::fmt::Debug for Mount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mount")
            .field("inner", &self.inner)
            .field("version", &self.version())
            .finish()
    }
}
