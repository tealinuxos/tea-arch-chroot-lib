use duct::cmd;
use std::io::Error;

pub async fn start_rsync() -> Result<(), Error>
{
    let command = r#"rsync
        -aAXv
        --exclude=/opt/tea-installer/*
        --exclude=/etc/motd
        --exclude=/etc/systemd/system/getty@tty1.service.d/*
        --exclude=/etc/mkinitcpio.conf.d/*
        --exclude=/etc/mkinitcpio.d/*
        --exclude=/dev/*
        --exclude=/proc/*
        --exclude=/sys/*
        --exclude=/tmp/*
        --exclude=/run/*
        --exclude=/mnt/*
        --exclude=/media/*
        --exclude=/var/cache/*
        --exclude=/
        --exclude=/home/*
        --exclude=/lost+found/*
        /
        /mnt"#;

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    cmd(&command[0], &command[1..]).run()?;

    Ok(())
}
