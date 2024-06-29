#[allow(unused_imports)]
use tea_arch_chroot_lib::resource::{ Timezones, Locales, FirmwareKind, Keyboard };
use tea_arch_chroot_lib::chroot::Timezone;
use tea_arch_chroot_lib::chroot::Account;
use tea_arch_chroot_lib::chroot::Locale;
use tea_arch_chroot_lib::chroot::pacman::{install_package, refresh_mirror};
use tea_arch_chroot_lib::chroot::bootloader::install_grub_bootloader;
use tea_arch_chroot_lib::prechroot::rsync::start_rsync;

#[tokio::main]
async fn main()
{
    println!("{}", serde_json::to_string_pretty(&Keyboard::list()).unwrap());
}

#[allow(dead_code)]
fn mirror()
{
    match refresh_mirror("Singapore")
    {
        Ok(_) => println!("Mirror refreshed"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

#[allow(dead_code)]
async fn rsync()
{
    match start_rsync().await
    {
        Ok(_) => println!("RSYNC success"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

#[allow(dead_code)]
fn bootloader()
{
    match install_grub_bootloader(FirmwareKind::UEFI, None, Some("/boot".to_string()))
    {
        Ok(_) => println!("GRUB successfully installed"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

#[allow(dead_code)]
fn timezone()
{
    let user = Timezone::new("Asia", "Jakarta");

    match user.generate_localtime()
    {
        Ok(_) => println!("Localtime successfully generated"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

#[allow(dead_code)]
fn locale()
{
    let locale = ["en_US.UTF-8 UTF-8", "id_ID.UTF-8 UTF-8"];

    let user = Locale::new(locale[0]);

    match user.set_locale()
    {
        Ok(_) => println!("Locale successfully set"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

#[allow(dead_code)]
fn account()
{
    let user = Account::new("Rustlix Slix", "rust2", "slixx", "whatever");

    match user.set_host()
    {
        Ok(_) => {

            println!("Successfully setting host");

            match user.add_user()
            {
                Ok(_) => println!("Successfully adding user"),
                Err(e) => panic!("Error: {:?}", e)
            }
        }

        Err(e) => panic!("Error: {:?}", e)
    }
}

#[allow(dead_code)]
fn pacman_install()
{
    let packages = vec!["fastfetch", "neofetch", "git", "giw"];

    match install_package(packages)
    {
        Ok(_) => println!("Successfully installed all packages"),
        Err(e) => panic!("Failed installing one or more package: {:?}", e)
    }
}
