use std::{fs, path::Path};

use crate::{
    errors::{Error, Result},
    initcli::utils,
    log, ENABLED_SERVICES_DIR,
};

pub fn disable(service: &str) -> Result<()> {
    // must be run as root
    utils::root::check_run_user()?;

    // path to the service symlink
    let symlink_service_path = format!("{ENABLED_SERVICES_DIR}/{service}");

    // check if already enabled
    if !Path::new(&symlink_service_path).exists() {
        Err(Error::ServiceAlreadyEnabled(service.to_string()))?
    }

    // delete enabled symlink
    fs::remove_file(symlink_service_path).map_err(Error::DeleteFile)?;

    log::success!("Service `{service}` disabled");

    Ok(())
}
