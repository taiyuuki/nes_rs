use super::*;

struct TestPPUBus {
    mem: [u8; 0x4000],
}

impl TestPPUBus {
    fn new() -> Self {
        Self { mem: [0; 0x4000] }
    }
}

impl PPUBus for TestPPUBus {
    fn ppu_read(&mut self, addr: u16) -> u8 {
        self.mem[(addr & 0x3FFF) as usize]
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        self.mem[(addr & 0x3FFF) as usize] = data;
    }
}

fn run_ppu_cycles(ppu: &mut PPU, bus: &mut TestPPUBus, cycles: usize) {
    for _ in 0..cycles {
        ppu.clock(bus);
    }
}

fn run_until_next_frame(ppu: &mut PPU, bus: &mut TestPPUBus) -> usize {
    let start_frame = ppu.frame();
    let mut cycles = 0;

    while ppu.frame() == start_frame {
        ppu.clock(bus);
        cycles += 1;
    }

    cycles
}

#[test]
fn ppudata_write_uses_configured_increment() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.cpu_write_register(&mut bus, 0x2000, CTRL_VRAM_INCREMENT);
    ppu.cpu_write_register(&mut bus, 0x2006, 0x20);
    ppu.cpu_write_register(&mut bus, 0x2006, 0x00);
    ppu.cpu_write_register(&mut bus, 0x2007, 0x12);
    ppu.cpu_write_register(&mut bus, 0x2007, 0x34);

    assert_eq!(bus.mem[0x2000], 0x12);
    assert_eq!(bus.mem[0x2020], 0x34);
}

#[test]
fn writing_ppuctrl_with_nmi_enabled_should_assert_nmi_during_vblank() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.status = STATUS_VBLANK;

    ppu.cpu_write_register(&mut bus, 0x2000, CTRL_NMI_ENABLE);

    assert!(ppu.nmi_line(), "NMI line should be asserted during VBlank");
}

#[test]
fn writing_ppuctrl_with_nmi_disabled_should_not_assert_nmi_during_vblank() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.status = STATUS_VBLANK;

    ppu.cpu_write_register(&mut bus, 0x2000, 0x00);

    assert!(
        !ppu.nmi_line(),
        "NMI line should remain low when PPUCTRL bit 7 is clear"
    );
}

#[test]
fn reading_ppustatus_should_clear_vblank_flag() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.status = STATUS_VBLANK;
    ppu.open_bus = 0x1B;

    let status = ppu.cpu_read_register(&mut bus, 0x2002);

    assert_eq!(status, 0x9B);
    assert!(!ppu.in_vblank(), "reading PPUSTATUS should clear VBlank");
}

#[test]
fn reading_ppustatus_should_reset_write_toggle() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.write_latch = true;

    let _ = ppu.cpu_read_register(&mut bus, 0x2002);

    assert!(
        !ppu.write_latch,
        "reading PPUSTATUS should clear the write toggle"
    );
}

#[test]
fn clock_enters_vblank_and_asserts_nmi_line_when_enabled() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.cpu_write_register(&mut bus, 0x2000, CTRL_NMI_ENABLE);

    for _ in 0..(341 * 262) {
        if ppu.nmi_line() {
            break;
        }
        ppu.clock(&mut bus);
    }

    assert!(ppu.in_vblank());
    assert!(ppu.nmi_line());
    assert_eq!(ppu.scanline(), 241);
}

#[test]
fn clock_clears_vblank_on_pre_render_scanline() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.status = STATUS_VBLANK | STATUS_SPRITE_ZERO_HIT | STATUS_SPRITE_OVERFLOW;

    ppu.clock(&mut bus);

    assert!(!ppu.in_vblank());
    assert_eq!(ppu.status & STATUS_SPRITE_ZERO_HIT, 0);
    assert_eq!(ppu.status & STATUS_SPRITE_OVERFLOW, 0);
}

#[test]
fn clock_renders_background_pixels_across_tile_boundaries() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.cpu_write_register(&mut bus, 0x2001, MASK_SHOW_BG | MASK_SHOW_BG_LEFTMOST);

    bus.mem[0x2000] = 0x01;
    bus.mem[0x2001] = 0x02;
    bus.mem[0x23C0] = 0x00;
    bus.mem[0x0010] = 0b1000_0000;
    bus.mem[0x0018] = 0x00;
    bus.mem[0x0020] = 0b1000_0000;
    bus.mem[0x0028] = 0x00;
    bus.mem[0x3F00] = 0x09;
    bus.mem[0x3F01] = 0x12;

    run_ppu_cycles(&mut ppu, &mut bus, 341 + 16);

    assert_eq!(ppu.bit_map[0], 0x12);
    assert_eq!(ppu.bit_map[1], 0x09);
    assert_eq!(ppu.bit_map[7], 0x09);
    assert_eq!(ppu.bit_map[8], 0x12);
}

#[test]
fn clock_uses_fine_x_scroll_for_background_pixels() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.cpu_write_register(&mut bus, 0x2001, MASK_SHOW_BG | MASK_SHOW_BG_LEFTMOST);
    ppu.cpu_write_register(&mut bus, 0x2005, 0x03);
    ppu.cpu_write_register(&mut bus, 0x2005, 0x00);

    bus.mem[0x2000] = 0x01;
    bus.mem[0x23C0] = 0x00;
    bus.mem[0x0010] = 0b1111_0000;
    bus.mem[0x0018] = 0x00;
    bus.mem[0x3F00] = 0x09;
    bus.mem[0x3F01] = 0x12;

    run_ppu_cycles(&mut ppu, &mut bus, 341 + 2);

    assert_eq!(ppu.bit_map[0], 0x12);
    assert_eq!(ppu.bit_map[1], 0x09);
}

#[test]
fn odd_frame_skips_one_dot_when_rendering_is_enabled() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    ppu.cpu_write_register(&mut bus, 0x2001, MASK_SHOW_BG);

    let initial_prerender = run_until_next_frame(&mut ppu, &mut bus);
    let odd_frame = run_until_next_frame(&mut ppu, &mut bus);
    let even_frame = run_until_next_frame(&mut ppu, &mut bus);

    assert_eq!(initial_prerender, 341);
    assert_eq!(odd_frame, 341 * 262 - 1);
    assert_eq!(even_frame, 341 * 262);
}

#[test]
fn odd_frame_does_not_skip_when_rendering_is_disabled() {
    let mut ppu = PPU::new();
    let mut bus = TestPPUBus::new();

    let initial_prerender = run_until_next_frame(&mut ppu, &mut bus);
    let first_frame = run_until_next_frame(&mut ppu, &mut bus);
    let second_frame = run_until_next_frame(&mut ppu, &mut bus);

    assert_eq!(initial_prerender, 341);
    assert_eq!(first_frame, 341 * 262);
    assert_eq!(second_frame, first_frame);
}
