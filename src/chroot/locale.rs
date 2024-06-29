use std::fs::File;
use duct::cmd;
use std::io::{ Write, Read, BufWriter, Error, BufReader  };
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Locale
{
    main: String,
    // locales: Vec<String>
}

impl Locale
{
    // pub fn new(main: &str, locales: Vec<&str>) -> Self
    pub fn new(main: &str) -> Self
    {
        Self
        {
            main: main.to_string(),
            // locales: locales.iter().map(|s| s.to_string()).collect::<Vec<String>>()
        }
    }

    pub fn set_locale(self) -> Result<(), Error>
    {
        match Self::write_locale(&self.main)
        {
            Ok(_) => {

                cmd!("arch-chroot", "/mnt", "locale-gen").run()?;
                Self::set_lang_variable(&self.main)?;

                Ok(())
            }

            Err(e) => Err(e)
        }
    }

    // fn write_locale(locales: Vec<String>) -> Result<(), Error>
    // {
    //     let locales = locales
    //         .iter()
    //         .map(|s| s.to_string() + "\n")
    //         .collect::<String>();
    //
    //     let locale_dot_gen = File::create("/mnt/etc/locale.gen");
    //
    //     match locale_dot_gen
    //     {
    //         Ok(mut file) => file.write_fmt(format_args!("{}", locales)),
    //         Err(e) => Err(e)
    //     }
    // }

    fn write_locale(locale: &str) -> Result<(), Error>
    {
        let locale_dot_gen = File::open("/mnt/etc/locale.gen")?;
        let mut locale_dot_gen = BufReader::new(locale_dot_gen);

        let mut before = String::new();

        locale_dot_gen.read_to_string(&mut before)?;

        let after = before
            .lines()
            .map(|line| if line.ends_with(locale) { format!("{}\n", line.replace("#", "")) } else { format!("{}\n", line) })
            .collect::<String>();

        let locale_dot_gen = File::create("/mnt/etc/locale.gen")?;
        let mut locale_dot_gen = BufWriter::new(locale_dot_gen);

        locale_dot_gen.write_fmt(format_args!("{}", after))?;

        Ok(())
    }

    fn set_lang_variable(locale: &str) -> Result<(), Error>
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
