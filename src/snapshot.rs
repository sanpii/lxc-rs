#[derive(Clone)]
pub struct Snapshot {
    inner: *mut lxc_sys::lxc_snapshot,
}

impl Snapshot {
    /**
     * Name of snapshot.
     */
    pub fn name(&self) -> String {
        prop!(self.name -> c_str)
    }

    /**
     * Full path to snapshots comment file.
     */
    pub fn comment_pathname(&self) -> Option<std::path::PathBuf> {
        let comment_pathname = unsafe { (*self.inner).comment_pathname };

        if comment_pathname.is_null() {
            None
        } else {
            Some(crate::ffi::to_string(comment_pathname).into())
        }
    }

    /**
     * Time snapshot was created.
     */
    pub fn timestamp(&self) -> String {
        prop!(self.timestamp -> c_str)
    }

    /**
     * Full path to LXCPATH for snapshot.
     */
    pub fn lxcpath(&self) -> std::path::PathBuf {
        prop!(self.lxcpath -> c_str).into()
    }

    /**
     * Returns a raw pointer to the snapshot.
     */
    pub fn as_ptr(&self) -> *const lxc_sys::lxc_snapshot {
        self.inner
    }

    /**
     * Returns a mutable raw pointer to the snapshot.
     */
    pub fn as_mut_ptr(&mut self) -> *mut lxc_sys::lxc_snapshot {
        self.inner
    }
}

#[doc(hidden)]
impl From<&*mut lxc_sys::lxc_snapshot> for Snapshot {
    fn from(inner: &*mut lxc_sys::lxc_snapshot) -> Self {
        Self { inner: *inner }
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        unsafe {
            if let Some(free) = (*self.inner).free {
                free(self.inner);
            }
        }
    }
}

impl std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Snapshot")
            .field("inner", &self.inner)
            .field("comment_pathname", &self.comment_pathname())
            .field("lxcpath", &self.lxcpath())
            .field("name", &self.name())
            .field("timestamp", &self.timestamp())
            .finish()
    }
}
