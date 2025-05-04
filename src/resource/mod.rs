use serde::{ Serialize, Deserialize };
pub mod timezone;
pub mod locale;
pub mod keyboard;

pub use self::timezone::Timezones;
pub use self::locale::Locales;
pub use self::keyboard::Keyboard;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FirmwareKind
{
    UEFI,
    BIOS
}
