use std::{os::unix::fs, path::Path};

use crate::{
    errors::{Error, Result},
    initcli::utils,
    log, ENABLED_SERVICES_DIR, SERVICES_DIR,
};

pub fn enable(service: &str) -> Result<()> {
    // must be run as root
    utils::root::check_run_user()?;

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

    log::success!("Service `{service}` enabled");

    Ok(())
}
