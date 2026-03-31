mod bus;
pub mod cartridge;
mod cpu;
mod dma;
mod ppu;

pub use cartridge::{Cartridge, CartridgeError, Mirroring};

pub struct NES {
    pub cpu: cpu::CPU,
    pub bus: bus::NESBus,
    master_clock: u64,
    cpu_ppu_counter: u8,
    cpu_schedule_index: usize,
}

impl NES {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::NESBus::new(),
            master_clock: 0,
            cpu_ppu_counter: 0,
            cpu_schedule_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.reset_cpu_schedule();
        self.cpu.reset(&mut self.bus);
        self.cpu.set_nmi(self.bus.ppu_nmi_line());
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.insert_cartridge(cartridge);
        self.reset_cpu_schedule();
    }

    pub fn load_cartridge_ines(&mut self, rom: &[u8]) -> Result<(), CartridgeError> {
        self.bus.load_cartridge_ines(rom)?;
        self.reset_cpu_schedule();
        Ok(())
    }

    pub fn clock(&mut self) {
        self.master_clock += 1;
        self.bus.tick_ppu();
        self.cpu.set_nmi(self.bus.ppu_nmi_line());

        let cpu_schedule = self.bus.ppu().cpu_schedule();
        self.cpu_ppu_counter += 1;
        if self.cpu_ppu_counter >= cpu_schedule[self.cpu_schedule_index] {
            self.cpu_ppu_counter = 0;
            self.cpu_schedule_index = (self.cpu_schedule_index + 1) % cpu_schedule.len();
            self.cpu.clock(&mut self.bus);
            self.cpu.set_nmi(self.bus.ppu_nmi_line());
        }
    }

    pub fn run_frame(&mut self) {
        let start_frame = self.bus.ppu_frame();
        while self.bus.ppu_frame() == start_frame {
            self.clock();
        }
    }

    pub fn master_clock(&self) -> u64 {
        self.master_clock
    }

    fn reset_cpu_schedule(&mut self) {
        self.cpu_ppu_counter = 0;
        self.cpu_schedule_index = 0;
    }
}

impl Default for NES {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::NES;

    fn make_ines_with_tv(flags9: u8) -> Vec<u8> {
        let mut rom = vec![0; 16];
        rom[0..4].copy_from_slice(b"NES\x1A");
        rom[4] = 1;
        rom[5] = 1;
        rom[9] = flags9;
        rom.extend(std::iter::repeat_n(0, 0x4000));
        rom.extend(std::iter::repeat_n(0, 0x2000));
        rom
    }

    #[test]
    fn run_frame_advances_exactly_one_ppu_frame() {
        let mut nes = NES::new();
        let start_clock = nes.master_clock();
        let start_frame = nes.bus.ppu().frame();

        nes.run_frame();

        assert_eq!(nes.bus.ppu().frame(), start_frame + 1);
        assert!(nes.master_clock() > start_clock);
    }

    #[test]
    fn pal_cpu_schedule_uses_33334_pattern() {
        let mut nes = NES::new();
        let rom = make_ines_with_tv(0x01);

        nes.load_cartridge_ines(&rom)
            .expect("PAL cartridge should load");

        for step in 1..=16 {
            nes.clock();
            let expected = match step {
                1..=2 => 0,
                3..=5 => 1,
                6..=8 => 2,
                9..=11 => 3,
                12..=15 => 4,
                16 => 5,
                _ => unreachable!(),
            };
            assert_eq!(nes.cpu.clocks(), expected, "master clock step {}", step);
        }
    }
}
