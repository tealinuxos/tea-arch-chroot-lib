use std::fs::File;
use duct::cmd;
use std::io::{ Write, Error };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Locale
{
    default: String,
    locales: Vec<String>
}

impl Locale
{
    pub fn new(default: &str, locales: Vec<&str>) -> Self
    {
        Self
        {
            default: default.to_string(),
            locales: locales.iter().map(|s| s.to_string()).collect::<Vec<String>>()
        }
    }

    pub fn set_locale(self) -> Result<(), Error>
    {
        match Self::write_locale(self.locales)
        {
            Ok(_) => {

                cmd!("arch-chroot", "/mnt", "locale-gen").run()?;
                Self::set_lang_variable(self.default)?;

                Ok(())
            }

            Err(e) => Err(e)
        }
    }

    fn write_locale(locales: Vec<String>) -> Result<(), Error>
    {
        let locales = locales
            .iter()
            .map(|s| s.to_string() + "\n")
            .collect::<String>();

        let locale_dot_gen = File::create("/mnt/etc/locale.gen");

        match locale_dot_gen
        {
            Ok(mut file) => file.write_fmt(format_args!("{}", locales)),
            Err(e) => Err(e)
        }
    }

    fn set_lang_variable(locale: String) -> Result<(), Error>
    {
        let locale = locale.split_whitespace().next().unwrap();
        let locale_dot_conf = File::create("/mnt/etc/locale.conf");

        match locale_dot_conf
        {
            Ok(mut file) => {
                file.write_fmt(format_args!("LANG={}", locale))?;

                Ok(())
            }

            Err(e) => Err(e)
        }
    }
}
