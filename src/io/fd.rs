//! Functions which operate on file descriptors.

use crate::{imp, io};
use io_lifetimes::{AsFd, IntoFd, OwnedFd};
#[cfg(all(libc, not(any(target_os = "wasi", target_os = "fuchsia"))))]
use std::ffi::OsString;

#[cfg(not(target_os = "wasi"))]
pub use imp::io::DupFlags;

/// `ioctl(fd, FIONREAD)`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_tty.2.html
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn ioctl_fionread<Fd: AsFd>(fd: &Fd) -> io::Result<u64> {
    let fd = fd.as_fd();
    imp::syscalls::ioctl_fionread(fd)
}

/// `isatty(fd)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/isatty.html
/// [Linux]: https://man7.org/linux/man-pages/man3/isatty.3.html
#[inline]
pub fn isatty<Fd: AsFd>(fd: &Fd) -> bool {
    let fd = fd.as_fd();
    imp::syscalls::isatty(fd)
}

/// Returns a pair of booleans indicating whether the file descriptor is
/// readable and/or writeable, respectively.
///
/// Unlike [`is_file_read_write`], this correctly detects whether sockets
/// have been shutdown, partially or completely.
///
/// [`is_file_read_write`]: crate::fs::is_file_read_write
#[cfg(not(target_os = "redox"))]
#[inline]
pub fn is_read_write<Fd: AsFd>(fd: &Fd) -> io::Result<(bool, bool)> {
    let fd = fd.as_fd();
    imp::syscalls::is_read_write(fd)
}

/// `dup(fd)`
///
/// Note that this does not set the `O_CLOEXEC` flag. To do a dup that does
/// set `O_CLOEXEC`, use [`fcntl_dupfd_cloexec`].
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [`fcntl_dupfd_cloexec`]: crate::fs::fcntl_dupfd_cloexec
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/dup.html
/// [Linux]: https://man7.org/linux/man-pages/man2/dup.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
pub fn dup<Fd: AsFd>(fd: &Fd) -> io::Result<OwnedFd> {
    let fd = fd.as_fd();
    imp::syscalls::dup(fd)
}

/// `dup3(fd, new, flags)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/dup2.html
/// [Linux]: https://man7.org/linux/man-pages/man2/dup2.2.html
#[cfg(not(target_os = "wasi"))]
#[inline]
#[doc(alias = "dup3")]
pub fn dup2<Fd: AsFd, NewFd: IntoFd>(fd: &Fd, new: NewFd, flags: DupFlags) -> io::Result<OwnedFd> {
    let fd = fd.as_fd();
    let new = new.into_fd();
    imp::syscalls::dup2(fd, new, flags)
}

/// `ttyname_r(fd)`
///
/// If `reuse` is non-empty, reuse its buffer to store the result if possible.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/ttyname.html
/// [Linux]: https://man7.org/linux/man-pages/man3/ttyname.3.html
// TODO: Implement ttyname for linux_raw
#[cfg(all(libc, not(any(target_os = "wasi", target_os = "fuchsia"))))]
#[inline]
pub fn ttyname<Fd: AsFd>(dirfd: &Fd, reuse: OsString) -> io::Result<OsString> {
    let dirfd = dirfd.as_fd();
    imp::syscalls::ttyname(dirfd, reuse)
}
