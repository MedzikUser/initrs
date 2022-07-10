use std::{ffi::CString, process};

use initrs::{errors::Error, init::services::Services, log, unwrap::Custom};
use nix::unistd;

const PATH_ENV: &str = "PATH=/sbin:/usr/sbin:/bin:/usr/bin";
const SHELL_ENV: &str = "SHELL=/bin/sh";

fn main() {
    // init muse be on PID 1
    if process::id() != 1 {
        log::error!("Init must be executed as PID 1");
        process::exit(1);
    }

    // Create new session and set process group id.
    unistd::setsid()
        .map_err(Error::SetSid)
        .expect_log("failed to set sid");

    // put env variables
    unsafe {
        libc::putenv(CString::new(PATH_ENV).unwrap().into_raw());
        libc::putenv(CString::new(SHELL_ENV).unwrap().into_raw());
    }

    // run services
    Services::new()
        .unwrap()
        .run()
        .expect_log("failed to run services");

    loop {
        let mut status = 0;

        unsafe {
            libc::waitpid(0, &mut status, 0);
        }
    }
}
