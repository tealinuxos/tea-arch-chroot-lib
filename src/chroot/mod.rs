pub mod pacman;
pub mod locale;
pub mod account;
pub mod timezone;
pub mod bootloader;
pub mod mkinitcpio;
pub mod keyboard;
pub mod os;
pub mod shell;

pub use self::account::Account;
pub use self::locale::Locale;
pub use self::timezone::Timezone;
