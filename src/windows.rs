use std::{
    ffi::{OsStr, OsString},
    os::windows::process::CommandExt,
    process::Command,
};

pub fn commands(path: impl AsRef<OsStr>) -> Vec<Command> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C")
        .arg("start")
        .raw_arg("\"\"")
        .raw_arg("/W")
        .raw_arg(wrap_quotes(path));
    vec![cmd]
}

pub fn with_command(path: impl AsRef<OsStr>, app: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new(app);
    cmd.arg(path);
    cmd
}

fn wrap_quotes(s: impl AsRef<OsStr>) -> OsString {
    let s = s.as_ref();
    let mut result = OsString::with_capacity(s.len() + 2);
    result.push("\"");
    result.push(s);
    result.push("\"");
    result
}
