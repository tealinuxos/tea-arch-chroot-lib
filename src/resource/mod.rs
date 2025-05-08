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

impl FirmwareKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            FirmwareKind::UEFI => "UEFI",
            FirmwareKind::BIOS => "BIOS",
        }
    }
}
  
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all="lowercase")]
pub enum MethodKind
{
    SINGLE,
    DUAL,
    MANUAL
}
