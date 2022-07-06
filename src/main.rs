pub mod command;
pub mod errors;
mod log;
mod mount;

use std::ffi::CString;

use libc::waitpid;
use nix::unistd;

use crate::errors::Error;

fn main() {
    better_panic::Settings::new()
        .message("Init panicked.")
        .verbosity(better_panic::Verbosity::Minimal)
        .install();

    // Create new session and set process group id.
    unistd::setsid()
        .map_err(Error::SetSid)
        .expect("failed to set sid");

    // put env variables
    unsafe {
        libc::putenv(
            CString::new("PATH=/sbin:/usr/sbin:/bin:/usr/bin")
                .unwrap()
                .into_raw(),
        );
        libc::putenv(CString::new("SHELL=/bin/sh").unwrap().into_raw());
    }

    // mount virtual filesystems
    mount::mount_vfs().expect("failed to mount filesystems");

    // run neofetch
    command::run("/usr/bin/neofetch").expect("failed to run neofetch");

    // start shell session
    command::run("/bin/sh").expect("failed to run shell session");

    loop {
        let mut status = 0;

        unsafe {
            waitpid(0, &mut status, 0);
        }
    }
}
