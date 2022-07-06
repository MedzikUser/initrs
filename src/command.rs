use std::process::Command;

use crate::{
    errors::{Error, Result},
    log,
};

/// Spawn and wait for command executed
pub fn run(line: &str) -> Result<()> {
    log::plus!("{}", line);

    let mut args = line.split(' ').map(|arg| arg.to_string());

    if let Some(cmd) = args.next() {
        // spawn command
        let mut child = Command::new(cmd)
            .args(args)
            .spawn()
            .map_err(Error::SpawnCommand)?;

        // wait for command EOF
        child.wait().map_err(Error::CommandWait)?;
    }

    Ok(())
}
