use alloc::{boxed::Box, collections::TryReserveError, ffi::CString, vec, vec::Vec};
use core::ffi::CStr;

use no_std_io::io::Result;

#[repr(transparent)]
pub struct Path {
    inner: CStr,
}
impl Path {
    pub fn new<P: AsRef<str>>(path: &P) -> &Self {
        let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(path.as_ref().as_bytes()) };
        unsafe { &*(cstr as *const CStr as *const Self) }
    }

    pub fn as_mut_cstr(&mut self) -> &mut CStr {
        &mut self.inner
    }

    pub fn as_cstr(&self) -> &CStr {
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
        let mut bytes = path.as_ref().as_cstr().to_bytes().to_vec();
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
