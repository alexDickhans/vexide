pub mod str;

use alloc::{string::String, vec::Vec};

use no_std_io::io::{Error, ErrorKind, Read, Result, Write};
use vex_sdk::{vexFileOpen, FRESULT};

use crate::{
    path::{Path, PathBuf},
    println,
};

fn fresult_to_io_error(fresult: FRESULT) -> Option<Error> {
    match fresult {
        FRESULT::FR_OK => None,
        FRESULT::FR_DISK_ERR => Some(Error::new(ErrorKind::Other, "hard disk error")),
        FRESULT::FR_INT_ERR => Some(Error::new(ErrorKind::Other, "internal assertion failed")),
        FRESULT::FR_NOT_READY => Some(Error::new(
            ErrorKind::Other,
            "the drive is not ready to work",
        )),
        FRESULT::FR_NO_FILE => Some(Error::new(ErrorKind::NotFound, "no such file")),
        FRESULT::FR_NO_PATH => Some(Error::new(ErrorKind::NotFound, "no such path")),
        FRESULT::FR_INVALID_NAME => Some(Error::new(
            ErrorKind::InvalidInput,
            "invalid path name format",
        )),
        FRESULT::FR_DENIED => Some(Error::new(
            ErrorKind::PermissionDenied,
            "access denied or directory full",
        )),
        FRESULT::FR_EXIST => Some(Error::new(ErrorKind::AlreadyExists, "access denied")),
        FRESULT::FR_INVALID_OBJECT => Some(Error::new(
            ErrorKind::InvalidData,
            "invalid file or directory object",
        )),
        FRESULT::FR_WRITE_PROTECTED => Some(Error::new(
            ErrorKind::PermissionDenied,
            "the drive is write protected",
        )),
        FRESULT::FR_INVALID_DRIVE => {
            Some(Error::new(ErrorKind::InvalidInput, "invalid drive number"))
        }
        FRESULT::FR_NOT_ENABLED => Some(Error::new(
            ErrorKind::Other,
            "drive not enabled/has no work area",
        )),
        FRESULT::FR_NO_FILESYSTEM => Some(Error::new(
            ErrorKind::NotFound,
            "drive has no FAT filesystem",
        )),
        FRESULT::FR_MKFS_ABORTED => Some(Error::new(
            ErrorKind::Other,
            "internal function f_mkfs encountered an error",
        )),
        FRESULT::FR_TIMEOUT => Some(Error::new(
            ErrorKind::TimedOut,
            "could not give access to the filesystem in time",
        )),
        FRESULT::FR_LOCKED => Some(Error::new(
            ErrorKind::PermissionDenied,
            "operation violated the file sharing policy",
        )),
        FRESULT::FR_NOT_ENOUGH_CORE => Some(Error::new(
            ErrorKind::Other,
            "LFN working buffer could not be allocated",
        )),
        FRESULT::FR_TOO_MANY_OPEN_FILES => Some(Error::new(
            ErrorKind::Other,
            "too many open files. you may only have 8 open files at a given time",
        )),
        FRESULT::FR_INVALID_PARAMETER => Some(Error::new(
            ErrorKind::InvalidInput,
            "invalid parameter given",
        )),
        _ => Some(Error::new(ErrorKind::Other, "unknown error")),
    }
}

pub(crate) fn valide_fs() -> Result<()> {
    let res = unsafe { vex_sdk::vexFileMountSD() };
    match fresult_to_io_error(res) {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

pub struct File {
    inner: *mut vex_sdk::FIL,
}
impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        valide_fs()?;
        let fd = unsafe {
            // mode is ignored by the sdk
            vexFileOpen(
                path.as_ref()
                    .as_fs_str()
                    .to_nul_terminated_bytes()
                    .as_ptr()
                    .cast(),
                c"".as_ptr(),
            )
        };

        if fd.is_null() {
            Err(Error::new(ErrorKind::NotFound, "file not found"))
        } else {
            Ok(Self { inner: fd })
        }
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<File> {
        valide_fs()?;

        let fd = unsafe {
            vex_sdk::vexFileOpenWrite(
                path.as_ref()
                    .as_fs_str()
                    .to_nul_terminated_bytes()
                    .as_ptr()
                    .cast(),
            )
        };

        if fd.is_null() {
            Err(Error::new(ErrorKind::NotFound, "file not found"))
        } else {
            Ok(Self { inner: fd })
        }
    }

    pub fn create_new<P: AsRef<Path>>(path: P) -> Result<File> {
        valide_fs()?;

        todo!()
    }
}
impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            vex_sdk::vexFileClose(self.inner);
        }
    }
}
impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        valide_fs()?;

        let buf_size = buf.len() as _;
        let ret = unsafe { vex_sdk::vexFileRead(buf.as_mut_ptr().cast(), 1, buf_size, self.inner) };

        Ok(ret as usize)
    }
}
impl Read for &File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        valide_fs()?;

        let buf_size = buf.len() as _;
        let ret = unsafe { vex_sdk::vexFileRead(buf.as_mut_ptr().cast(), 1, buf_size, self.inner) };

        Ok(ret as usize)
    }
}
impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        valide_fs()?;

        let ret = unsafe {
            vex_sdk::vexFileWrite(
                buf.as_ptr().cast_mut().cast(),
                1,
                buf.len() as _,
                self.inner,
            )
        };

        if ret == -1 {
            Err(Error::new(ErrorKind::Other, "write error"))
        } else if ret == 0 {
            Err(Error::new(
                ErrorKind::WriteZero,
                "could not write any bytes",
            ))
        } else {
            Ok(ret as usize)
        }
    }

    fn flush(&mut self) -> Result<()> {
        // We have no buffers for now
        Ok(())
    }
}
impl Write for &File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        valide_fs()?;

        let ret = unsafe {
            vex_sdk::vexFileWrite(
                buf.as_ptr().cast_mut().cast(),
                1,
                buf.len() as _,
                self.inner,
            )
        };

        if ret == -1 {
            Err(Error::new(ErrorKind::Other, "write error"))
        } else {
            Ok(ret as usize)
        }
    }

    fn flush(&mut self) -> Result<()> {
        // We have no buffers for now
        Ok(())
    }
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    File::create(path.as_ref())?.write_all(contents.as_ref())
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    read(path).map(|v| String::from_utf8(v).unwrap())
}
