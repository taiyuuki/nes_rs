use crate::dma::DmaController;
use crate::ppu::{PPU, PpuBus};

pub trait CPUBus {
    fn cpu_read(&mut self, addr: u16) -> u8;
    fn cpu_write(&mut self, addr: u16, data: u8);
    fn try_dma(&mut self) -> bool {
        false
    }

    fn cpu_read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.cpu_read(addr) as u16;
        let hi = self.cpu_read(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
}

struct PpuMemory {
    chr_ram: [u8; 0x2000],
    vram: [u8; 0x800],
    palette: [u8; 0x20],
}

impl PpuMemory {
    fn new() -> Self {
        Self {
            chr_ram: [0; 0x2000],
            vram: [0; 0x800],
            palette: [0; 0x20],
        }
    }

    fn normalize_addr(addr: u16) -> u16 {
        addr & 0x3FFF
    }

    fn palette_index(addr: u16) -> usize {
        let mut index = (addr - 0x3F00) & 0x001F;
        if matches!(index, 0x10 | 0x14 | 0x18 | 0x1C) {
            index -= 0x10;
        }
        index as usize
    }
}

impl Default for PpuMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuBus for PpuMemory {
    fn ppu_read(&mut self, addr: u16) -> u8 {
        let addr = Self::normalize_addr(addr);
        match addr {
            0x0000..=0x1FFF => self.chr_ram[addr as usize],
            0x2000..=0x3EFF => self.vram[((addr - 0x2000) & 0x07FF) as usize],
            0x3F00..=0x3FFF => self.palette[Self::palette_index(addr)],
            _ => 0,
        }
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        let addr = Self::normalize_addr(addr);
        match addr {
            0x0000..=0x1FFF => self.chr_ram[addr as usize] = data,
            0x2000..=0x3EFF => self.vram[((addr - 0x2000) & 0x07FF) as usize] = data,
            0x3F00..=0x3FFF => self.palette[Self::palette_index(addr)] = data,
            _ => {}
        }
    }
}

pub struct NESBus {
    pub ram: [u8; 0x800],
    ppu: PPU,
    ppu_memory: PpuMemory,
    dma: DmaController,
    // Additional components: APU, cartridge, etc. can be added here
}

impl NESBus {
    pub fn new() -> Self {
        NESBus {
            ram: [0; 0x800],
            ppu: PPU::new(),
            ppu_memory: PpuMemory::new(),
            dma: DmaController::new(),
        }
    }

    pub fn set_ram(&mut self, data: [u8; 0x800]) {
        self.ram = data;
    }

    pub fn ppu(&self) -> &PPU {
        &self.ppu
    }

    pub fn reset(&mut self) {
        self.ppu.reset();
        self.dma = DmaController::new();
    }

    pub fn tick_ppu(&mut self) {
        let ppu = &mut self.ppu;
        let ppu_memory = &mut self.ppu_memory;
        ppu.clock(ppu_memory);
    }

    pub fn ppu_nmi_line(&self) -> bool {
        self.ppu.nmi_line()
    }

    pub fn dma_in_progress(&self) -> bool {
        self.dma.in_progress()
    }

    pub(crate) fn dma_read(&mut self, addr: u16) -> u8 {
        self.cpu_read_internal(addr)
    }

    pub(crate) fn dma_write_oam(&mut self, data: u8) {
        self.ppu.write_oam_dma(data);
    }

    fn cpu_read_internal(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize],
            0x2000..=0x3FFF => {
                let ppu = &mut self.ppu;
                let ppu_memory = &mut self.ppu_memory;
                ppu.cpu_read_register(ppu_memory, 0x2000 | (addr & 0x0007))
            }
            // Handle other address ranges (APU, cartridge, etc.)
            _ => 0,
        }
    }

    fn cpu_write_internal(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize] = data,
            0x2000..=0x3FFF => {
                let ppu = &mut self.ppu;
                let ppu_memory = &mut self.ppu_memory;
                ppu.cpu_write_register(ppu_memory, 0x2000 | (addr & 0x0007), data);
            }
            0x4014 => self.dma.request_oam_dma(data),
            // Handle other address ranges (APU, cartridge, etc.)
            _ => {}
        }
    }

    fn tick_dma_cpu_cycle(&mut self) -> bool {
        let mut dma = std::mem::take(&mut self.dma);
        let consumed = dma.tick_cpu_cycle(self);
        self.dma = dma;
        consumed
    }
}

impl CPUBus for NESBus {
    fn cpu_read(&mut self, addr: u16) -> u8 {
        self.cpu_read_internal(addr)
    }

    fn cpu_write(&mut self, addr: u16, data: u8) {
        self.cpu_write_internal(addr, data);
    }

    fn try_dma(&mut self) -> bool {
        self.tick_dma_cpu_cycle()
    }
}

impl Default for NESBus {
    fn default() -> Self {
        Self::new()
    }
}
