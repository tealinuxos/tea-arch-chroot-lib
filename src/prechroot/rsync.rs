use duct::cmd;

pub fn start_rsync() -> Option<()>
{
    let command = r#"rsync
        -aAXv
        --exclude=/etc/motd
        --exclude=/etc/systemd/system/getty@tty1.service.d/*
        --exclude=/etc/mkinitcpio.conf
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
        --exclude=/lost+found/*
        /
        /mnt"#;

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    let rsync = cmd(&command[0], &command[1..]).run();

    match rsync
    {
        Ok(_) => Some(()),
        Err(_) => None
    }
}
