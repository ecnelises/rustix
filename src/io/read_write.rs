//! `read` and `write`, optionally positioned, optionally vectored

use crate::{imp, io};
use io_lifetimes::AsFd;
use std::io::{IoSlice, IoSliceMut};

/// `RWF_*` constants.
#[cfg(any(linux_raw, all(libc, target_os = "linux", target_env = "gnu")))]
pub use imp::io::ReadWriteFlags;

/// `read(fd, buf)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/read.html
/// [Linux]: https://man7.org/linux/man-pages/man2/read.2.html
#[inline]
pub fn read<Fd: AsFd>(fd: &Fd, buf: &mut [u8]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::read(fd, buf)
}

/// `write(fd, buf)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/write.html
/// [Linux]: https://man7.org/linux/man-pages/man2/write.2.html
#[inline]
pub fn write<Fd: AsFd>(fd: &Fd, buf: &[u8]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::write(fd, buf)
}

/// `pread(fd, buf, offset)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/pread.html
/// [Linux]: https://man7.org/linux/man-pages/man2/pread.2.html
#[inline]
pub fn pread<Fd: AsFd>(fd: &Fd, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pread(fd, buf, offset)
}

/// `pwrite(fd, bufs)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/pwrite.html
/// [Linux]: https://man7.org/linux/man-pages/man2/pwrite.2.html
#[inline]
pub fn pwrite<Fd: AsFd>(fd: &Fd, buf: &[u8], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pwrite(fd, buf, offset)
}

/// `readv(fd, bufs)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/readv.html
/// [Linux]: https://man7.org/linux/man-pages/man2/readv.2.html
#[inline]
pub fn readv<Fd: AsFd>(fd: &Fd, bufs: &[IoSliceMut]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::readv(fd, bufs)
}

/// `writev(fd, bufs)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/writev.html
/// [Linux]: https://man7.org/linux/man-pages/man2/writev.2.html
#[inline]
pub fn writev<Fd: AsFd>(fd: &Fd, bufs: &[IoSlice]) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::writev(fd, bufs)
}

/// `preadv(fd, bufs, offset)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/preadv.2.html
#[inline]
#[cfg(not(target_os = "redox"))]
pub fn preadv<Fd: AsFd>(fd: &Fd, bufs: &[IoSliceMut], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::preadv(fd, bufs, offset)
}

/// `pwritev(fd, bufs, offset)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/pwritev.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn pwritev<Fd: AsFd>(fd: &Fd, bufs: &[IoSlice], offset: u64) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pwritev(fd, bufs, offset)
}

/// `preadv2(fd, bufs, offset, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/preadv2.2.html
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
#[inline]
pub fn preadv2<Fd: AsFd>(
    fd: &Fd,
    bufs: &[IoSliceMut],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::preadv2(fd, bufs, offset, flags)
}

/// `pwritev2(fd, bufs, offset, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/pwritev2.2.html
#[cfg(any(
    linux_raw,
    all(
        libc,
        target_pointer_width = "64",
        target_os = "linux",
        target_env = "gnu"
    )
))]
#[inline]
pub fn pwritev2<Fd: AsFd>(
    fd: &Fd,
    bufs: &[IoSlice],
    offset: u64,
    flags: ReadWriteFlags,
) -> io::Result<usize> {
    let fd = fd.as_fd();
    imp::syscalls::pwritev2(fd, bufs, offset, flags)
}
