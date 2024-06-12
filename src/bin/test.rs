use tea_arch_chroot_lib::resource::timezone::Timezones;
use tea_arch_chroot_lib::resource::locale::Locales;
use tea_arch_chroot_lib::chroot::timezone::Timezone;
use tea_arch_chroot_lib::chroot::account::Account;
use tea_arch_chroot_lib::chroot::locale::Locale;
use tea_arch_chroot_lib::chroot::pacman::install_package;

fn main()
{
    println!("{}", Timezones::list_json());
    println!("{}", Locales::list_json());

    // timezone();
}

fn timezone()
{
    let user = Timezone::new("Asia", "Jakarta");

    match user.generate_localtime()
    {
        Ok(_) => println!("Localtime successfully generated"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

fn locale()
{
    let locale = vec!["en_US.UTF-8 UTF-8", "id_ID.UTF-8 UTF-8"];

    let user = Locale::new(locale[0], locale);

    match user.set_locale()
    {
        Ok(_) => println!("Locale successfully set"),
        Err(e) => panic!("Error: {:#?}", e)
    }
}

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

fn pacman_install()
{
    let packages = vec!["fastfetch", "neofetch", "git", "giw"];

    match install_package(packages)
    {
        Ok(_) => println!("Successfully installed all packages"),
        Err(e) => panic!("Failed installing one or more package: {:?}", e)
    }
}