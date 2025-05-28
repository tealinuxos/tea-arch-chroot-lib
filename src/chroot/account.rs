use duct::cmd;
use std::fs::File;
use std::io::{ Error, Write };
use std::path::Path;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Account
{
    pub fullname: String,
    pub username: String,
    pub hostname: String,
    password: String
}

impl Account
{
    pub fn new(fullname: &str, username: &str, hostname: &str, password: &str) -> Self
    {
        Self
        {
            fullname: fullname.to_string(),
            username: username.to_string(),
            hostname: hostname.to_string(),
            password: password.to_string()
        }
    }

    pub fn set_host(&self) -> Result<(), Error>
    {
        match Self::write_hostname(&self.hostname)
        {
            Ok(_) => {

                Self::set_root_password(&self.password)?;

                Ok(())
            }

            Err(e) => Err(e)
        }
    }

    pub fn add_user(&self) -> Result<(), Error>
    {
        let command = cmd!("arch-chroot", "/tealinux-mount", "useradd", "-mG", "wheel", &self.username).run();

        match command
        {
            Ok(_) => {

                match Self::set_user_password(&self.username, &self.password)
                {
                    Ok(_) => {

                        Self::set_user_fullname(&self.username, &self.fullname)?;

                        Ok(())
                    }

                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    fn write_hostname(hostname: &str) -> Result<(), Error>
    {
        let file = File::create("/tealinux-mount/etc/hostname");

        match file
        {
            Ok(mut file) => {

                file.write_fmt(format_args!("{}", hostname))?;

                Ok(())
            }

            Err(e) => Err(e)
        }
    }

    fn set_root_password(password: &str) -> Result<(), Error>
    {
        cmd!("arch-chroot", "/tealinux-mount", "passwd", "--stdin").stdin_bytes(password).run()?;

        Ok(())
    }

    fn set_user_password(username: &str, password: &str) -> Result<(), Error>
    {
        cmd!("arch-chroot", "/tealinux-mount", "passwd", username, "--stdin").stdin_bytes(password).run()?;

        Ok(())
    }

    fn set_user_fullname(username: &str, fullname: &str) -> Result<(), Error>
    {
        cmd!("arch-chroot", "/tealinux-mount", "chfn", "--full-name", fullname, username).run()?;

        Ok(())
    }

    pub fn remove_user(username: &str) -> Result<(), Error>
    {
        cmd!("arch-chroot", "/tealinux-mount", "userdel", "--remove", username).run()?;

        Ok(())
    }
    
    pub fn set_cosmic_automatic_login(&self) -> Result<(), Error>
    {
        let config_path = Path::new("/etc/greetd/config.toml");

        let init = format!(r#"[initial_session]
command = "start-cosmic"
user = "{}"
"#, self.username);

        if std::fs::exists(config_path)?
        {
            let mut config_toml = std::fs::read_to_string(config_path)?;
            config_toml.push_str(&init);

            let mut config_file = File::create(config_path)?;
            config_file.write_fmt(format_args!("{}", config_toml))?;
        }
        else
        {
            let mut config_file = File::create(config_path)?;
            config_file.write_fmt(format_args!("{}", init))?;
        }

        Ok(())
    }
}
