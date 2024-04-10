use alloc::{string::String, vec::Vec};
use core::fmt::Debug;

#[repr(transparent)]
pub struct FsStr {
    inner: [u8],
}

impl FsStr {
    pub fn new<S: AsRef<FsStr>>(s: &S) -> &Self {
        unsafe { &s.as_ref() }
    }

    pub fn to_str(&self) -> &str {
        // SAFETY: `OsStr` is guaranteed to be valid UTF-8 because it is always constructed from valid UTF-8
        unsafe { core::str::from_utf8_unchecked(&self.inner) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub unsafe fn from_encoded_bytes_unchecked(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes as *const [u8] as *const FsStr) }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.inner
    }

    pub fn to_nul_terminated_bytes<'a>(&'a self) -> Vec<u8> {
        let mut bytes = Vec::from(&self.inner);
        bytes.push(0);
        bytes
    }

    pub fn is_ascii(&self) -> bool {
        self.inner.iter().all(|&c| c.is_ascii())
    }
}

impl Debug for FsStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl AsRef<FsStr> for str {
    fn as_ref(&self) -> &FsStr {
        unsafe { &*(self.as_bytes() as *const [u8] as *const FsStr) }
    }
}

impl AsRef<FsStr> for String {
    fn as_ref(&self) -> &FsStr {
        unsafe { &*(self.as_bytes() as *const [u8] as *const FsStr) }
    }
}
