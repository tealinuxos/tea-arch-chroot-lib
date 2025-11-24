use serde::{Deserialize, Serialize};
pub mod keyboard;
pub mod locale;
pub mod timezone;

pub use self::keyboard::Keyboard;
pub use self::locale::Locales;
pub use self::timezone::Timezones;
use specta::Type as SpectaType;
use std::default::Default;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, SpectaType)]
pub enum FirmwareKind {
    UEFI,
    BIOS,
}

impl FirmwareKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            FirmwareKind::UEFI => "UEFI",
            FirmwareKind::BIOS => "BIOS",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, SpectaType)]
#[serde(rename_all = "lowercase")]
pub enum MethodKind {
    SINGLE,
    DUAL,
    MANUAL,
}

impl Default for MethodKind {
    fn default() -> Self {
        MethodKind::SINGLE
    }
}
