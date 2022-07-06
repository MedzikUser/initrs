use crate::{command, errors::Result};

pub fn mount_tmpfs() -> Result<()> {
    // mount /proc
    command::run("mount -t proc proc /proc")?;

    // mount /dev
    command::run("mount -t devtmpfs devtmpfs /dev")?;

    // mount /sys
    command::run("mount -t sysfs sysfs /sys")?;

    // mount /run
    command::run("mount -t tmpfs tmpfs /run")?;

    Ok(())
}
