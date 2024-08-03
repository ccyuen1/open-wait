use std::{ffi::OsStr, process::Command};

#[cfg(target_os = "windows")]
use windows as os;

#[cfg(target_os = "macos")]
use macos as os;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "aix",
    target_os = "hurd"
))]
use unix as os;

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "aix",
    target_os = "hurd"
)))]
compile_error!("open-wait is not supported on this platform");

pub fn commands(path: impl AsRef<OsStr>) -> Vec<Command> {
    os::commands(path)
}

pub fn with_command(path: impl AsRef<OsStr>, app: impl AsRef<OsStr>) -> Command {
    os::with_command(path, app)
}

#[cfg(windows)]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "solaris",
    target_os = "aix",
    target_os = "hurd"
))]
mod unix;
