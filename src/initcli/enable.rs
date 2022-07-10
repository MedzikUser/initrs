use std::{os::unix::fs, path::Path};

use nix::unistd::Uid;

use crate::{
    errors::{Error, Result},
    log, ENABLED_SERVICES_DIR, SERVICES_DIR,
};

pub fn enable(service: &str) -> Result<()> {
    // must be run as root
    if !Uid::effective().is_root() {
        Err(Error::AccessDenied)?
    }

    // path to the service
    let service_path = format!("{SERVICES_DIR}/{service}.toml");

    // check if service exists
    if !Path::new(&service_path).exists() {
        Err(Error::ServiceNotFound(service.to_string()))?
    }

    // path to the service symlink
    let symlink_service_path = format!("{ENABLED_SERVICES_DIR}/{service}");

    // check if already enabled
    if Path::new(&symlink_service_path).exists() {
        Err(Error::ServiceAlreadyEnabled(service.to_string()))?
    }

    // create symlink
    fs::symlink(service_path, symlink_service_path).map_err(Error::CreateSymlink)?;

    log::command!("Service `{service}` enabled");

    Ok(())
}
