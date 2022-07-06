use crate::{command, errors::Result};

/// Mount virtual filesystems
pub fn mount_vfs() -> Result<()> {
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
