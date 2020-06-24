use crate::root;
use log::info;
use std::io;
use std::process::Command;

pub fn reboot() -> Result<(), io::Error> {
    info!("Reboot requested");
    let mut cmd = Command::new("reboot");
    root::execute_as_root(|| match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.is_empty() {
                return Ok(());
            }
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    })?
}

pub fn poweroff() -> Result<(), io::Error> {
    info!("Poweroff requested");
    let mut cmd = Command::new("poweroff");
    root::execute_as_root(|| match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.is_empty() {
                return Ok(());
            }
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    })?
}
