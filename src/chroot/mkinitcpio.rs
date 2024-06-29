use duct::cmd;
use std::io::Error;

pub fn generate_initramfs(preset: &str) -> Result<(), Error>
{
    cmd!("mkinitcpio", "--preset", preset).run()?;

    Ok(())
}
