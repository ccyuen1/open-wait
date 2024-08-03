use std::{ffi::OsStr, process::Command};

pub fn commands(path: impl AsRef<OsStr>) -> Vec<Command> {
    let mut cmd = Command::new("/usr/bin/open");
    cmd.arg("-W").arg("-n").arg(path.as_ref());
    vec![cmd]
}

pub fn with_command(path: impl AsRef<OsStr>, app: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new("/usr/bin/open");
    cmd.arg("-W")
        .arg("-n")
        .arg("-a")
        .arg(app.as_ref())
        .arg(path.as_ref());
    cmd
}
