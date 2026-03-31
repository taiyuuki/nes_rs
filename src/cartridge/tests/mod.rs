use super::*;

fn make_ines(prg_banks: u8, chr_banks: u8, flags6: u8, prg_fill: u8, chr_fill: u8) -> Vec<u8> {
    let mut rom = vec![0; INES_HEADER_LEN];
    rom[0..4].copy_from_slice(b"NES\x1A");
    rom[4] = prg_banks;
    rom[5] = chr_banks;
    rom[6] = flags6;

    rom.extend(std::iter::repeat_n(
        prg_fill,
        prg_banks as usize * PRG_BANK_LEN,
    ));
    rom.extend(std::iter::repeat_n(
        chr_fill,
        chr_banks as usize * CHR_BANK_LEN,
    ));
    rom
}

#[test]
fn parses_ines_header_and_maps_nrom_prg() {
    let mut rom = make_ines(1, 1, 0x01, 0xEA, 0x55);
    let prg_start = INES_HEADER_LEN;
    rom[prg_start] = 0x78;
    rom[prg_start + 0x3FFF] = 0x4C;

    let mut cartridge = Cartridge::from_ines(&rom).expect("valid NROM should parse");

    assert_eq!(cartridge.mirroring(), Mirroring::Vertical);
    assert_eq!(cartridge.cpu_read(0x8000), Some(0x78));
    assert_eq!(cartridge.cpu_read(0xBFFF), Some(0x4C));
    assert_eq!(cartridge.cpu_read(0xC000), Some(0x78));
}

#[test]
fn allocates_chr_ram_when_chr_banks_are_zero() {
    let rom = make_ines(1, 0, 0x00, 0xEA, 0x00);
    let mut cartridge = Cartridge::from_ines(&rom).expect("CHR RAM cartridge should parse");

    assert_eq!(cartridge.ppu_read(0x000A), Some(0x00));
    assert!(cartridge.ppu_write(0x000A, 0x9C));
    assert_eq!(cartridge.ppu_read(0x000A), Some(0x9C));
}

#[test]
fn rejects_unsupported_mapper() {
    let rom = make_ines(1, 1, 0x10, 0x00, 0x00);

    let err = match Cartridge::from_ines(&rom) {
        Ok(_) => panic!("mapper 1 should be rejected"),
        Err(err) => err,
    };

    assert_eq!(err, CartridgeError::UnsupportedMapper(1));
}
