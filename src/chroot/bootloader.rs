use crate::resource::FirmwareKind;
use std::io::Error;
use duct::cmd;
use std::path::Path;

pub fn get_firmware_type() -> FirmwareKind
{
    let path = Path::new("/sys/firmware/efi/");

    match path.exists()
    {
        true => FirmwareKind::UEFI,
        _ => FirmwareKind::BIOS
    }
}

pub fn install_grub_bootloader(firmware_kind: FirmwareKind, disk: Option<String>, efi: Option<String>) -> Result<(), Error>
{
    let command = match firmware_kind
    {
        FirmwareKind::UEFI => {

            let efi_dirs = format!("--efi-directory={}", efi.expect("EFI not specified"));
            cmd!("arch-chroot", "/mnt", "grub-install", "--target=x86_64-efi", efi_dirs, "--bootloader-id=TealinuxOS")
        }

        FirmwareKind::BIOS => {
            cmd!("arch-chroot", "/mnt", "grub-install", "--target=i386-pc", disk.expect("Disk not specified"))
        }
    };

    match command.run()
    {
        Ok(_) => {

            self::grub_mkconfig()?;
            Ok(())
        }

        Err(e) => Err(e)
    }
}

pub fn grub_mkconfig() -> Result<(), Error>
{
    cmd!("arch-chroot", "/mnt", "grub-mkconfig", "-o", "/boot/grub/grub.cfg").run()?;

    Ok(())
}
