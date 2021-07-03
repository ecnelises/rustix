//! Functions which operate on file descriptors.

use crate::{imp, io};
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))]
use imp::fs::FallocateFlags;
#[cfg(not(target_os = "wasi"))]
use imp::fs::Mode;
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))]
// not implemented in libc for netbsd yet
use imp::fs::StatFs;
use imp::{fs::Stat, time::Timespec};
use io_lifetimes::{AsFd, BorrowedFd};
use std::io::SeekFrom;

/// `lseek(fd, offset, whence)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/lseek.html
/// [Linux]: https://man7.org/linux/man-pages/man2/lseek.2.html
#[inline]
pub fn seek<Fd: AsFd>(fd: &Fd, pos: SeekFrom) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::seek(fd, pos)
}

/// `lseek(fd, 0, SEEK_CUR)`
///
/// Return the current position of the file descriptor. This is a subset of
/// the functionality of `seek`, but this interface makes it easier for users
/// to declare their intent not to mutate any state.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/lseek.html
/// [Linux]: https://man7.org/linux/man-pages/man2/lseek.2.html
#[inline]
pub fn tell<Fd: AsFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::tell(fd)
}

/// `fchmod(fd)`.
///
/// Note that this implementation does not support `O_PATH` file descriptors,
/// even on platforms where the host libc emulates it.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fchmod.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fchmod.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn fchmod<Fd: AsFd>(fd: &Fd, mode: Mode) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fchmod(fd, mode)
}

/// `fstat(fd)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fstat.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fstat.2.html
#[inline]
pub fn fstat<Fd: AsFd>(fd: &Fd) -> io::Result<Stat> {
    let fd = fd.as_fd();
    imp::syscalls::fstat(fd)
}

/// `fstatfs(fd)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/fstatfs.2.html
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "wasi")))] // not implemented in libc for netbsd yet
#[inline]
pub fn fstatfs<Fd: AsFd>(fd: &Fd) -> io::Result<StatFs> {
    let fd = fd.as_fd();
    imp::syscalls::fstatfs(fd)
}

/// `futimens(fd, times)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/futimens.html
/// [Linux]: https://man7.org/linux/man-pages/man2/futimens.2.html
#[inline]
pub fn futimens<Fd: AsFd>(fd: &Fd, times: &[Timespec; 2]) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::futimens(fd, times)
}

/// `fallocate(fd, mode, offset, len)`
///
/// This is a more general form of `posix_fallocate`, adding a `mode` argument
/// which modifies the behavior. On platforms which only support
/// `posix_fallocate` and not the more general form, no `FallocateFlags` values
/// are defined so it will always be empty.
///
/// # References
///  - [POSIX]
///  - [Linux `fallocate`]
///  - [Linux `posix_fallocate`]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/posix_fallocate.html
/// [Linux `fallocate`]: https://man7.org/linux/man-pages/man2/fallocate.2.html
/// [Linux `posix_fallocate`]: https://man7.org/linux/man-pages/man3/posix_fallocate.3.html
#[cfg(not(any(target_os = "netbsd", target_os = "redox", target_os = "openbsd")))] // not implemented in libc for netbsd yet
#[inline]
#[doc(alias = "posix_fallocate")]
pub fn fallocate<Fd: AsFd>(fd: &Fd, mode: FallocateFlags, offset: u64, len: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fallocate(fd, mode, offset, len)
}

/// `fcntl(fd, F_GETFL) & O_ACCMODE`.
///
/// Returns a pair of booleans indicating whether the file descriptor is
/// readable and/or writeable, respectively. This is only reliable on files;
/// for example, it doesn't reflect whether sockets have been shut down; for
/// general I/O handle support, use [`io::is_read_write`].
#[inline]
pub fn is_file_read_write<Fd: AsFd>(fd: &Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_fd();
    _is_file_read_write(fd)
}

pub(crate) fn _is_file_read_write(fd: BorrowedFd<'_>) -> io::Result<(bool, bool)> {
    let mode = imp::syscalls::fcntl_getfl(fd)?;

    // Check for `O_PATH`.
    #[cfg(any(
        target_os = "android",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "emscripten"
    ))]
    if mode.contains(crate::fs::OFlags::PATH) {
        return Ok((false, false));
    }

    // Use `RWMODE` rather than `ACCMODE` as `ACCMODE` may include `O_PATH`.
    // We handled `O_PATH` above.
    match mode & crate::fs::OFlags::RWMODE {
        crate::fs::OFlags::RDONLY => Ok((true, false)),
        crate::fs::OFlags::RDWR => Ok((true, true)),
        crate::fs::OFlags::WRONLY => Ok((false, true)),
        _ => unreachable!(),
    }
}

/// `fsync(fd)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fsync.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fsync.2.html
#[inline]
pub fn fsync<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fsync(fd)
}

/// `fdatasync(fd)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/fdatasync.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fdatasync.2.html
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "redox")))]
#[inline]
pub fn fdatasync<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::fdatasync(fd)
}

/// `ftruncate(fd, length)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/ftruncate.html
/// [Linux]: https://man7.org/linux/man-pages/man2/ftruncate.2.html
#[inline]
pub fn ftruncate<Fd: AsFd>(fd: &Fd, length: u64) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::ftruncate(fd, length)
}
