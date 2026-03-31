mod nrom;

use self::nrom::Nrom;
use super::CartridgeError;

pub(super) trait Mapper {
    fn cpu_read(&mut self, addr: u16) -> Option<u8>;
    fn cpu_write(&mut self, addr: u16, data: u8) -> bool;
    fn ppu_read(&mut self, addr: u16) -> Option<u8>;
    fn ppu_write(&mut self, addr: u16, data: u8) -> bool;
    fn check_a12(&mut self, _addr: u16) {}
}

pub(super) fn from_mapper_id(
    mapper_id: u16,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
) -> Result<Box<dyn Mapper>, CartridgeError> {
    match mapper_id {
        0 => Ok(Box::new(Nrom::new(prg_rom, chr_rom))),
        _ => Err(CartridgeError::UnsupportedMapper(mapper_id)),
    }
}
