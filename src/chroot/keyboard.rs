use std::io::{BufWriter, Error, Write};
use std::fs::{create_dir_all, File};
use std::path::Path;

pub struct Keyboard
{
    layout: String,
    variant: String
}

impl Keyboard
{
    pub fn new(layout: &str, variant: &str) -> Self
    {
        Self
        {
            layout: layout.to_string(),
            variant: variant.to_string()
        }
    }

    fn write_cosmic_xkb(self, username: &str) -> Result<(), Error>
    {
        let path = format!("/tealinux-mount/home/{}/.config/cosmic/com.system76.CosmicComp/v1/", username);
        let path = Path::new(&path);

        let xkb_config = if !Path::exists(Path::new(path))
        {
            create_dir_all(path)?;
            File::create(format!("{}/xkb_config" ,path.display()))?
        }
        else
        {
            File::create(format!("{}/xkb_config", path.display()))?
        };

        let config = format!(r#"(
    rules: ""
    model: ""
    layout: "{}",
    variant: "{}",
    options: ""
)
            "#, self.layout, self.variant);

        let mut xkb_config_writer = BufWriter::new(xkb_config);

        xkb_config_writer.write_fmt(format_args!("{}", config))?;

        Ok(())
    }

    pub fn set_keymap_cosmic(self, username: &str) -> Result<(), Error>
    {
        match self.write_cosmic_xkb(username)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
