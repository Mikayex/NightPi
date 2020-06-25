use crate::root;
use log::info;
use std::io;
use std::process::{Command, Output};

fn execute_command(cmd: &mut Command) -> Result<Output, io::Error> {
    match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.is_empty() {
                return Ok(output);
            }
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    }
}

pub fn reboot() -> Result<(), io::Error> {
    info!("Reboot requested");
    let mut cmd = Command::new("reboot");
    root::execute_as_root(|| execute_command(&mut cmd))?.and(Ok(()))
}

pub fn poweroff() -> Result<(), io::Error> {
    info!("Poweroff requested");
    let mut cmd = Command::new("poweroff");
    root::execute_as_root(|| execute_command(&mut cmd))?.and(Ok(()))
}

pub fn ssh_enabled() -> Result<bool, io::Error> {
    let mut cmd = Command::new("systemctl");
    cmd.arg("is-active").arg("--quiet").arg("ssh.service");
    let output = cmd.output()?;
    Ok(output.status.success())
}

pub fn set_ssh_enabled(enabled: bool) -> Result<(), io::Error> {
    if ssh_enabled()? == enabled {
        return Ok(());
    }

    let mut cmd = Command::new("systemctl");
    if enabled {
        cmd.arg("start").arg("--quiet").arg("ssh.service");
        info!("Starting SSH");
    } else {
        cmd.arg("stop").arg("--quiet").arg("ssh.service");
        info!("Stopping SSH");
    }
    root::execute_as_root(|| execute_command(&mut cmd))?.and(Ok(()))
}
