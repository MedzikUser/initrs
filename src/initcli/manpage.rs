use std::io;

use clap::App;
use clap_mangen::Man;

use crate::errors::{Error, Result};

pub fn manpage(commands: App) -> Result<()> {
    let man = Man::new(commands);

    man.render(&mut io::stdout())
        .map_err(Error::GenerateManpage)?;

    Ok(())
}
