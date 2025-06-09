use std::io::Error;

pub fn change_shell(user: &str, shell_path: &str) -> Result<(), Error>
{
    let _chsh = duct::cmd!("arch-chroot", "/tealinux-mount", "chsh", "--shell", shell_path, user).run()?;

    Ok(())
}
