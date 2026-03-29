mod bus;
mod cpu;
mod dma;
mod ppu;

pub struct NES {
    pub cpu: cpu::CPU,
    pub bus: bus::NESBus,
    master_clock: u64,
}

impl NES {
    pub fn new() -> Self {
        Self {
            cpu: cpu::CPU::new(),
            bus: bus::NESBus::new(),
            master_clock: 0,
        }
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.cpu.reset(&mut self.bus);
        self.cpu.set_nmi(self.bus.ppu_nmi_line());
    }

    pub fn clock(&mut self) {
        self.master_clock += 1;
        self.bus.tick_ppu();
        self.cpu.set_nmi(self.bus.ppu_nmi_line());

        if self.master_clock % 3 == 0 {
            self.cpu.cpu_clock(&mut self.bus);
            self.cpu.set_nmi(self.bus.ppu_nmi_line());
        }
    }

    pub fn master_clock(&self) -> u64 {
        self.master_clock
    }
}

impl Default for NES {
    fn default() -> Self {
        Self::new()
    }
}
