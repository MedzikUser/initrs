use crate::{command, errors::Result, log};

/// Mount according to /etc/fstab
pub fn local_mount() -> Result<()> {
    log::plus!("Mounting filesystems mentioned in /etc/fstab");

    // mount all filesystems mentioned in fstab
    command::run("mount -a")?;

    Ok(())
}
