use duct::cmd;
use std::io::Error;

pub fn regenerate_pacman_key() -> Result<(), Error>
{
    let command = "arch-chroot /mnt pacman-key --init";

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    let init = cmd(&command[0], &command[1..]).run();

    match init
    {
        Ok(_) => {

            let command = "arch-chroot /mnt pacman-key --populate";

            let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

            cmd(&command[0], &command[1..]).run()?;

            Ok(())
        }

        Err(e) => Err(e)
    }
}

pub fn update_packages() -> Result<(), Error>
{
    let command = "arch-chroot /mnt pacman -Syyu --ask 4";

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    cmd(&command[0], &command[1..]).run()?;

    Ok(())
}

pub fn install_package(packages: Vec<&str>) -> Result<(), Error>
{
    for package in packages
    {
        cmd!("pacman", "-S", "--needed", "--noconfirm", package).run()?;
    }

    Ok(())
}
