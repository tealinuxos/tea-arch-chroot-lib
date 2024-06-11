use duct::cmd;
use std::io::Error;

pub struct Timezone
{
    region: String,
    city: String
}

impl Timezone
{
    pub fn new(region: &str, city: &str) -> Self
    {
        Self
        {
            region: region.to_string(),
            city: city.to_string()
        }
    }

    pub fn generate_localtime(&self) -> Result<(), Error>
    {
        let command = format!("arch-chroot /mnt ln -sf /usr/share/zoneinfo/{}/{} /etc/localtime", self.region, self.city);

        let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

        let _ = cmd(&command[0], &command[1..]).run()?;

        Ok(())
    }
}
