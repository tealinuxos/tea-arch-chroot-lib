use serde::{ Serialize, Deserialize };
pub mod timezone;
pub mod locale;
pub mod keyboard;

pub use self::timezone::Timezones;
pub use self::locale::Locales;
pub use self::keyboard::Keyboard;
use std::default::Default;

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
  
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum MethodKind
{
    SINGLE,
    DUAL,
    MANUAL
}

impl Default for MethodKind {
    fn default() -> Self {
        MethodKind::SINGLE
    }
}