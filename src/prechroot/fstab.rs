use duct::cmd;
use std::fs::File;

pub fn generate_fstab() -> Option<()>
{
    let file = File::create("/mnt/etc/fstab").expect("Failed to create fstab file");

    let command = "genfstab -U /mnt";

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    let generate = cmd(&command[0], &command[1..]).stdout_file(file).run();

    match generate
    {
        Ok(_) => Some(()),
        Err(_) => None
    }
}

