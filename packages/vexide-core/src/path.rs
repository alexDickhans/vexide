use alloc::{boxed::Box, collections::TryReserveError, ffi::CString, vec, vec::Vec};
use core::ffi::CStr;
use crate::fs::str::FsStr;


#[repr(transparent)]
pub struct Path {
    inner: FsStr,
}
impl Path {
    pub fn new<P: AsRef<FsStr>>(path: &P) -> &Self {
        unsafe { &*(path.as_ref() as *const FsStr as *const Self) }
    }

    pub fn as_mut_fs_str(&mut self) -> &mut FsStr {
        &mut self.inner
    }

    pub fn as_fs_str(&self) -> &FsStr {
        &self.inner
    }
}

impl AsRef<Path> for &str {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}
impl AsRef<Path> for &Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

pub struct PathBuf {
    inner: Vec<u8>,
}
impl PathBuf {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        let mut bytes = path.as_ref().as_fs_str().to_bytes().to_vec();
        self.inner.append(&mut bytes);
    }

    pub fn pop(&mut self) {
        self.inner.pop();
    }

    pub fn as_cstring(&self) -> CString {
        let mut bytes = vec![0u8; self.inner.len() + 1];
        bytes[..self.inner.len()].copy_from_slice(&self.inner);
        // Safety: `bytes` is guaranteed to be nul-terminated
        unsafe { CString::from_vec_unchecked(bytes) }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> core::result::Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> core::result::Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}
