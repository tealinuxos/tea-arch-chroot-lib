use duct::cmd;
use std::io::Error;

pub fn generate_initramfs(preset: &str) -> Result<(), Error>
{
    cmd!("arch-chroot", "/tealinux-mount", "mkinitcpio", "--preset", preset).run()?;

    Ok(())
}
