mod bus;
mod cpu;

pub struct NES {
    pub cpu: cpu::CPU,
    pub bus: bus::NESBus,
    // pub ppu: ppu::PPU, // PPU can be added later
}