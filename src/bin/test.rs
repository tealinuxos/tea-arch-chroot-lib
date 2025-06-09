use tea_arch_chroot_lib::chroot::os::Os;
#[allow(unused_imports)]
use tea_arch_chroot_lib::resource::{ Timezones, Locales, FirmwareKind, Keyboard };
use tea_arch_chroot_lib::chroot::Timezone;
use tea_arch_chroot_lib::chroot::Account;
use tea_arch_chroot_lib::chroot::Locale;
use tea_arch_chroot_lib::chroot::shell;
use tea_arch_chroot_lib::chroot::pacman::{install_package, refresh_mirror};
use tea_arch_chroot_lib::chroot::bootloader::install_grub_bootloader;
use tea_arch_chroot_lib::prechroot::rsync::start_rsync;
use tea_arch_chroot_lib::chroot::keyboard;

#[tokio::main]
async fn main()
{
    // println!("{}", serde_json::to_string_pretty(&Keyboard::list()).unwrap());
    // locale();
    // os();
    // keyboard();
    // account();
    shell();
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
    // let locale = ["en_US.UTF-8 UTF-8", "id_ID.UTF-8 UTF-8"];

    let user = Locale::new("en_US.UTF-8 UTF-8");

    // match user.set_locale()
    // {
    //     Ok(_) => println!("Locale successfully set"),
    //     Err(e) => panic!("Error: {:#?}", e)
    // }

    println!("{}", user.get_main_locale());
}

#[allow(dead_code)]
fn account()
{
    let user = Account::new("Rustlix Slix", "rust2", "slixx", "whatever", false);

    match user.set_cosmic_automatic_login()
    {
        Ok(_) => {
            println!("Successfully set autologin");
        },
        Err(e) => {
            println!("Failed to set autologin {}", e);
        }
    }

    // match user.set_host()
    // {
    //     Ok(_) => {
    //
    //         println!("Successfully setting host");
    //
    //         match user.add_user()
    //         {
    //             Ok(_) => println!("Successfully adding user"),
    //             Err(e) => panic!("Error: {:?}", e)
    //         }
    //     }
    //
    //     Err(e) => panic!("Error: {:?}", e)
    // }
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

#[allow(dead_code)]
fn keyboard()
{
    let keyb = keyboard::Keyboard::new("us".to_string(), None);

    match keyb.set_keymap_cosmic(false, "ssa".to_string())
    {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e)
    }
}

#[allow(dead_code)]
fn os()
{
    println!("{:#?}", Os::get_other_os())
}

#[allow(dead_code)]
fn shell()
{
    println!("{:#?}", shell::change_shell("ssa", "/usr/bin/fish"));
}
