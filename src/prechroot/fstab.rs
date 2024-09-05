use duct::cmd;
use std::fs::File;
use std::io::Error;

pub async fn generate_fstab() -> Result<(), Error>
{
    let file = File::create("/tealinux-mount/etc/fstab").expect("Failed to create fstab file");

    let command = "genfstab -U /tealinux-mount";

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    cmd(&command[0], &command[1..]).stdout_file(file).run()?;

    Ok(())
}

