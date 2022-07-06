use crate::{command, errors::Result, log};

/// Mount virtual filesystems
pub fn mount_vfs() -> Result<()> {
    log::plus!("Mounting virtual filesystems");

    // mount /proc
    command::run("mount -n -t proc proc /proc")?;

    // mount /dev
    command::run("mount -n -t devtmpfs devtmpfs /dev")?;

    // mount /sys
    command::run("mount -n -t sysfs sysfs /sys")?;

    // mount /run
    command::run("mount -n -t tmpfs tmpfs /run")?;

    Ok(())
}
