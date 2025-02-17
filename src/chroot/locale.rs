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

    pub fn get_main_locale(self) -> String
    {
        self.main
    }

    pub fn set_locale(self) -> Result<(), Error>
    {
        Self::write_locale("en_US.UTF-8 UTF-8")?;
        match Self::write_locale(&self.main)
        {
            Ok(_) => {

                cmd!("arch-chroot", "/tealinux-mount", "locale-gen").run()?;
                Self::set_locale_dot_conf(&self.main)?;

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
    //     let locale_dot_gen = File::create("/tealinux-mount/etc/locale.gen");
    //
    //     match locale_dot_gen
    //     {
    //         Ok(mut file) => file.write_fmt(format_args!("{}", locales)),
    //         Err(e) => Err(e)
    //     }
    // }

    fn write_locale(locale: &str) -> Result<(), Error>
    {
        let locale_dot_gen = File::open("/tealinux-mount/etc/locale.gen")?;
        let mut locale_dot_gen = BufReader::new(locale_dot_gen);

        let mut before = String::new();

        locale_dot_gen.read_to_string(&mut before)?;

        let after = before
            .lines()
            .map(|line| if line.trim().ends_with(locale) { format!("{}\n", line.replace("#", "")) } else { format!("{}\n", line) })
            .collect::<String>();

        let locale_dot_gen = File::create("/tealinux-mount/etc/locale.gen")?;
        let mut locale_dot_gen = BufWriter::new(locale_dot_gen);

        locale_dot_gen.write_fmt(format_args!("{}", after))?;

        Ok(())
    }

    fn set_locale_dot_conf(locale: &str) -> Result<(), Error>
    {
        let locale = locale.split_whitespace().next().unwrap();
        let locale_dot_conf = File::create("/tealinux-mount/etc/locale.conf");

        match locale_dot_conf
        {
            Ok(mut file) => {

                file.write_fmt(format_args!("LANG=en_US.UTF-8\n"))?;
                file.write_fmt(format_args!("LC_ADDRESS={}\n", locale))?;
                file.write_fmt(format_args!("LC_IDENTIFICATION={}\n", locale))?;
                file.write_fmt(format_args!("LC_MEASUREMENT={}\n", locale))?;
                file.write_fmt(format_args!("LC_MONETARY={}\n", locale))?;
                file.write_fmt(format_args!("LC_NAME={}\n", locale))?;
                file.write_fmt(format_args!("LC_NUMERIC={}\n", locale))?;
                file.write_fmt(format_args!("LC_PAPER={}\n", locale))?;
                file.write_fmt(format_args!("LC_TELEPHONE={}\n", locale))?;
                file.write_fmt(format_args!("LC_TIME={}\n", locale))?;

                Ok(())
            }

            Err(e) => Err(e)
        }
    }
}
