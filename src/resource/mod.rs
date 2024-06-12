pub mod timezone;
pub mod locale;

pub use self::timezone::Timezones;
pub use self::locale::Locales;

pub enum FirmwareKind
{
    UEFI,
    BIOS
}
