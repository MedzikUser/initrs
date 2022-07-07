use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    command,
    errors::{Error, Result},
    log,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "Unit")]
    pub unit: ServiceUnit,
    #[serde(rename = "Service")]
    pub service: ServiceService,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceUnit {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "After")]
    pub after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceService {
    #[serde(rename = "Exec")]
    pub exec: String,
}

impl Service {
    pub fn new(path: PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path).unwrap();
        Ok(toml::from_str(&content).unwrap())
    }

    pub fn find_and_execute() -> Result<()> {
        log::plus!("Running services");

        let paths = fs::read_dir("/etc/initrs/services/").map_err(Error::ReadServicesDir)?;

        for path in paths {
            let path = path.map_err(Error::ReadServicesDir)?;

            let service = Service::new(path.path())?;

            command::run(&service.service.exec)?;
        }

        Ok(())
    }
}
