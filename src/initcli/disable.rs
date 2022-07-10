use std::{fs, path::Path};

use nix::unistd::Uid;

use crate::{
    errors::{Error, Result},
    log, ENABLED_SERVICES_DIR,
};

pub fn disable(service: &str) -> Result<()> {
    // must be run as root
    if !Uid::effective().is_root() {
        Err(Error::AccessDenied)?
    }

    // path to the service symlink
    let symlink_service_path = format!("{ENABLED_SERVICES_DIR}/{service}");

    // check if already enabled
    if !Path::new(&symlink_service_path).exists() {
        Err(Error::ServiceAlreadyEnabled(service.to_string()))?
    }

    // delete enabled symlink
    fs::remove_file(symlink_service_path).map_err(Error::DeleteFile)?;

    log::command!("Service `{service}` disabled");

    Ok(())
}
