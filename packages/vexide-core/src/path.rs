use core::fmt::Debug;

use alloc::{boxed::Box, collections::TryReserveError, ffi::CString, string::String, vec, vec::Vec};

use crate::fs::str::FsStr;

pub struct Components<'a> {
    inner: Vec<&'a FsStr>,
}
impl<'a> Iterator for Components<'a> {
    type Item = &'a FsStr;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

#[repr(transparent)]
#[derive(Debug)]
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

    pub fn iter<'a>(&'a self) -> Components<'a> {
        let components: Vec<_> = self.inner.to_str().split("/").map(|component| component.as_ref()).collect();
        Components { inner: components }
    }

    pub fn file_name(&self) -> Option<&FsStr> {
        let Some(end) = self.iter().last() else {
            return None;
        };
        let end = end.to_str();
        let extension_len = end.chars().rev().take_while(|c| *c != '.').count() + 1;
        let file_name = &end[..end.len() - extension_len];
        Some(file_name.as_ref())
    }

    pub fn into_path_buf(self: Box<Self>) -> PathBuf {
        PathBuf {
            inner: self.inner.to_bytes().to_vec(),
        }
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
impl Debug for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.inner))
    }
}
