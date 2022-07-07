use std::process::Command;

use crate::{
    errors::{Error, Result},
    log,
};

/// Spawn and wait for command executed
pub fn run(line: &str) -> Result<()> {
    log::command!("{}", line);

    let status = Command::new("sh")
        .args(vec!["-c", line])
        .status()
        .map_err(Error::RunCommand)?;

    if !status.success() {
        Err(Error::ExitCommand(status.code().unwrap_or(-1)))?
    }

    Ok(())
}

/// Spawn without waiting for command EOF
pub fn spawn(line: &str) -> Result<()> {
    log::command!("{}", line);

    let mut args = line.split(' ').map(|arg| arg.to_string());

    if let Some(cmd) = args.next() {
        let status = Command::new(cmd)
            .args(args)
            .status()
            .map_err(Error::RunCommand)?;

        if !status.success() {
            Err(Error::ExitCommand(status.code().unwrap_or(-1)))?
        }
    }

    Ok(())
}
