use std::{ffi::OsStr, process::Command};

pub fn commands(path: impl AsRef<OsStr>) -> Vec<Command> {
    let mut cmd = Command::new("mimeopen");
    cmd.arg("-n");
    cmd.arg(path.as_ref());
    vec![cmd]
}

pub fn with_command(path: impl AsRef<OsStr>, app: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new(app.as_ref());
    cmd.arg(path.as_ref());
    cmd
}
