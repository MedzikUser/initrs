use std::ffi::CString;

use initrs::{command, errors::Error, mount, unwrap::Custom};
use nix::unistd;

const PATH_ENV: &str = "PATH=/sbin:/usr/sbin:/bin:/usr/bin";
const SHELL_ENV: &str = "SHELL=/bin/sh";

fn main() {
    better_panic::Settings::new()
        .message("Init panicked.")
        .verbosity(better_panic::Verbosity::Minimal)
        .install();

    // Create new session and set process group id.
    unistd::setsid()
        .map_err(Error::SetSid)
        .expect_log("failed to set sid");

    // put env variables
    unsafe {
        libc::putenv(CString::new(PATH_ENV).unwrap().into_raw());
        libc::putenv(CString::new(SHELL_ENV).unwrap().into_raw());
    }

    // mount virtual filesystems
    mount::mount_vfs().expect_log("failed to mount file systems");

    // mount according to /etc/fstab
    mount::local_mount().expect_log("failed to mount filesystems mentioned in fstab");

    // run neofetch
    command::run("/usr/bin/neofetch").expect_log("failed to run neofetch");

    // start shell session
    command::run("/bin/sh").expect_log("failed to run shell session");

    // loop {
    //     let mut status = 0;

    //     unsafe {
    //         libc::waitpid(0, &mut status, 0);
    //     }
    // }
}
