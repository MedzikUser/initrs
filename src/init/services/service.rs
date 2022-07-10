use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    errors::{Error, Result},
    init::command,
    log,
    unwrap::Custom,
};

/// The path where the services to be launched are located.
pub const SERVICES_PATH: &str = "/etc/initrs/services/";

/// Service schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// The service name
    #[serde(rename = "Name")]
    pub name: String,
    /// The service type
    #[serde(rename = "Type")]
    pub typ: String,
    /// Run the service after another
    #[serde(rename = "After")]
    pub after: Option<Vec<String>>,
    /// The command that starts the service
    #[serde(rename = "ExecStart")]
    pub exec_start: Vec<String>,
    /// The command that stopped the service
    #[serde(rename = "ExecStop")]
    pub exec_stop: Option<Vec<String>>,
}

impl Service {
    /// Parse service config
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        // read the service file into a string
        let content = fs::read_to_string(path).map_err(Error::OpenServiceFile)?;
        // parse the service fi;e
        toml::from_str(&content).map_err(Error::ParseServiceFile)
    }

    /// Run the service function
    pub fn run(&self) -> Result<()> {
        log::plus!("Starting service: {}", self.name);

        // run all commands
        for cmd in self.exec_start.iter() {
            command::run(cmd).unwrap_log();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Services {
    /// `Vec` where it is located all services
    pub services: Vec<Service>,
}

impl Services {
    /// Parse the configuration of all services
    pub fn new() -> Result<Self> {
        // get paths of the services file
        let paths = fs::read_dir(SERVICES_PATH).map_err(Error::ReadServicesDir)?;

        let mut services = Vec::new();

        // parse services from files
        for path in paths {
            let path = path.map_err(Error::ReadServicesDir)?;

            let service = Service::new(path.path())?;

            services.push(service);
        }

        Ok(Services { services })
    }

    /// Run all services
    pub fn run(&self) -> Result<()> {
        // `Vec` which contains all the names of the services executed
        let mut executed = Vec::new();

        for service in self.services.iter() {
            // if the service has been executed skip it
            if executed.contains(&service.name) {
                continue;
            }

            // run all required services before this
            if let Some(depends) = &service.after {
                self.check_depends_and_run(depends, &mut executed);
            }

            // run service
            service.run().unwrap_log();
            // push service to executed
            executed.push(service.name.clone())
        }

        Ok(())
    }

    fn check_depends_and_run(&self, depends: &Vec<String>, executed: &mut Vec<String>) {
        for depend in depends {
            // if the service hasn't executed before
            if !executed.contains(depend) {
                // get dependency index in Vec
                let index = self
                    .services
                    .iter()
                    .position(|x| &x.name == depend)
                    .unwrap();

                // check dependencies of the service
                if let Some(depends) = &self.services[index].after {
                    self.check_depends_and_run(depends, executed);
                }

                // run service
                self.services[index].run().unwrap_log();

                // push service to executed
                executed.push(depend.clone())
            }
        }
    }
}
