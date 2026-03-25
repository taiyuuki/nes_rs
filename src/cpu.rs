use crate::bus::CPUBus;

#[derive(Clone, Copy)]
pub enum AddrMode {
    IMM,
    IMP,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
    JAM, // Illegal addressing mode
}

#[derive(Clone, Copy)]
pub enum OpCode {
    BRK,
    ORA,
    XXX,
    ASO,
    NOP,
    ASL,
    PHP,
    BPL,
    CLC,
    NII,
    JSR,
    AND,
    RLA,
    BIT,
    ROL,
    PLP,
    BMI,
    SEC,
    RTI,
    EOR,
    LSE,
    LSR,
    PHA,
    JMP,
    BVC,
    CLI,
    RTS,
    ADC,
    RRA,
    ROR,
    PLA,
    BVS,
    SEI,
    STA,
    AXS,
    STY,
    STX,
    DEY,
    TXA,
    BCC,
    AXA,
    TYA,
    TXS,
    LDY,
    LDA,
    LDX,
    LAX,
    TAY,
    TAX,
    BCS,
    CLV,
    TSX,
    CPY,
    CMP,
    DCM,
    DEC,
    INY,
    DEX,
    BNE,
    CLD,
    CPX,
    SBC,
    INS,
    INC,
    INX,
    BEQ,
    SED,
}

#[derive(Clone, Copy)]
pub struct Inst(OpCode, AddrMode, u8); // opcode, addressing mode, cycles

const INST_SET: [Inst; 256] = [
    Inst(OpCode::BRK, AddrMode::IMP, 7),
    Inst(OpCode::ORA, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::ASO, AddrMode::IZX, 8),
    Inst(OpCode::NOP, AddrMode::ZP0, 3),
    Inst(OpCode::ORA, AddrMode::ZP0, 3),
    Inst(OpCode::ASL, AddrMode::ZP0, 5),
    Inst(OpCode::ASO, AddrMode::ZP0, 5),
    Inst(OpCode::PHP, AddrMode::IMP, 3),
    Inst(OpCode::ORA, AddrMode::IMM, 2),
    Inst(OpCode::ASL, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::NOP, AddrMode::ABS, 4),
    Inst(OpCode::ORA, AddrMode::ABS, 4),
    Inst(OpCode::ASL, AddrMode::ABS, 6),
    Inst(OpCode::ASO, AddrMode::ABS, 6),
    Inst(OpCode::BPL, AddrMode::REL, 2),
    Inst(OpCode::ORA, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::ASO, AddrMode::IZY, 8),
    Inst(OpCode::NOP, AddrMode::ZPX, 4),
    Inst(OpCode::ORA, AddrMode::ZPX, 4),
    Inst(OpCode::ASL, AddrMode::ZPX, 6),
    Inst(OpCode::ASO, AddrMode::ZPX, 6),
    Inst(OpCode::CLC, AddrMode::IMP, 2),
    Inst(OpCode::ORA, AddrMode::ABY, 4),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::ASO, AddrMode::ABY, 7),
    Inst(OpCode::NOP, AddrMode::ABX, 4),
    Inst(OpCode::ORA, AddrMode::ABX, 4),
    Inst(OpCode::ASL, AddrMode::ABX, 7),
    Inst(OpCode::ASO, AddrMode::ABX, 7),
    Inst(OpCode::JSR, AddrMode::ABS, 6),
    Inst(OpCode::AND, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::RLA, AddrMode::IZX, 8),
    Inst(OpCode::BIT, AddrMode::ZP0, 3),
    Inst(OpCode::AND, AddrMode::ZP0, 3),
    Inst(OpCode::ROL, AddrMode::ZP0, 5),
    Inst(OpCode::RLA, AddrMode::ZP0, 5),
    Inst(OpCode::PLP, AddrMode::IMP, 4),
    Inst(OpCode::AND, AddrMode::IMM, 2),
    Inst(OpCode::ROL, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::BIT, AddrMode::ABS, 4),
    Inst(OpCode::AND, AddrMode::ABS, 2),
    Inst(OpCode::ROL, AddrMode::ABS, 6),
    Inst(OpCode::RLA, AddrMode::ABS, 6),
    Inst(OpCode::BMI, AddrMode::REL, 2),
    Inst(OpCode::AND, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::RLA, AddrMode::IZY, 8),
    Inst(OpCode::NOP, AddrMode::ZPX, 4),
    Inst(OpCode::AND, AddrMode::ZPX, 4),
    Inst(OpCode::ROL, AddrMode::ZPX, 6),
    Inst(OpCode::RLA, AddrMode::ZPX, 6),
    Inst(OpCode::SEC, AddrMode::IMP, 2),
    Inst(OpCode::AND, AddrMode::ABY, 4),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::RLA, AddrMode::ABY, 7),
    Inst(OpCode::NOP, AddrMode::ABX, 4),
    Inst(OpCode::AND, AddrMode::ABX, 4),
    Inst(OpCode::ROL, AddrMode::ABX, 7),
    Inst(OpCode::RLA, AddrMode::ABX, 7),
    Inst(OpCode::RTI, AddrMode::IMP, 6),
    Inst(OpCode::EOR, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::LSE, AddrMode::IZX, 8),
    Inst(OpCode::NOP, AddrMode::ZP0, 3),
    Inst(OpCode::EOR, AddrMode::ZP0, 3),
    Inst(OpCode::LSR, AddrMode::ZP0, 5),
    Inst(OpCode::LSE, AddrMode::ZP0, 5),
    Inst(OpCode::PHA, AddrMode::IMP, 3),
    Inst(OpCode::EOR, AddrMode::IMM, 2),
    Inst(OpCode::LSR, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::JMP, AddrMode::ABS, 3),
    Inst(OpCode::EOR, AddrMode::ABS, 4),
    Inst(OpCode::LSR, AddrMode::ABS, 6),
    Inst(OpCode::LSE, AddrMode::ABS, 6),
    Inst(OpCode::BVC, AddrMode::REL, 2),
    Inst(OpCode::EOR, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::LSE, AddrMode::IZY, 8),
    Inst(OpCode::NOP, AddrMode::ZPX, 4),
    Inst(OpCode::EOR, AddrMode::ZPX, 4),
    Inst(OpCode::LSR, AddrMode::ZPX, 6),
    Inst(OpCode::LSE, AddrMode::ZPX, 6),
    Inst(OpCode::CLI, AddrMode::IMP, 2),
    Inst(OpCode::EOR, AddrMode::ABY, 4),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::LSE, AddrMode::ABY, 7),
    Inst(OpCode::NOP, AddrMode::ABX, 4),
    Inst(OpCode::EOR, AddrMode::ABX, 4),
    Inst(OpCode::LSR, AddrMode::ABX, 7),
    Inst(OpCode::LSE, AddrMode::ABX, 7),
    Inst(OpCode::RTS, AddrMode::IMP, 6),
    Inst(OpCode::ADC, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::RRA, AddrMode::IZX, 8),
    Inst(OpCode::NOP, AddrMode::ZP0, 3),
    Inst(OpCode::ADC, AddrMode::ZP0, 3),
    Inst(OpCode::ROR, AddrMode::ZP0, 5),
    Inst(OpCode::RRA, AddrMode::ZP0, 5),
    Inst(OpCode::PLA, AddrMode::IMP, 4),
    Inst(OpCode::ADC, AddrMode::IMM, 2),
    Inst(OpCode::ROR, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::JMP, AddrMode::IND, 5),
    Inst(OpCode::ADC, AddrMode::ABS, 4),
    Inst(OpCode::ROR, AddrMode::ABS, 6),
    Inst(OpCode::RRA, AddrMode::ABS, 6),
    Inst(OpCode::BVS, AddrMode::REL, 2),
    Inst(OpCode::ADC, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::RRA, AddrMode::IZY, 8),
    Inst(OpCode::NOP, AddrMode::ZPX, 4),
    Inst(OpCode::ADC, AddrMode::ZPX, 4),
    Inst(OpCode::ROR, AddrMode::ZPX, 6),
    Inst(OpCode::RRA, AddrMode::ZPX, 6),
    Inst(OpCode::SEI, AddrMode::IMP, 2),
    Inst(OpCode::ADC, AddrMode::ABY, 4),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::RRA, AddrMode::ABY, 7),
    Inst(OpCode::NOP, AddrMode::ABX, 4),
    Inst(OpCode::ADC, AddrMode::ABX, 4),
    Inst(OpCode::ROR, AddrMode::ABX, 7),
    Inst(OpCode::RRA, AddrMode::ABX, 7),
    Inst(OpCode::NII, AddrMode::IMM, 2),
    Inst(OpCode::STA, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::AXS, AddrMode::IZX, 6),
    Inst(OpCode::STY, AddrMode::ZP0, 3),
    Inst(OpCode::STA, AddrMode::ZP0, 3),
    Inst(OpCode::STX, AddrMode::ZP0, 3),
    Inst(OpCode::AXS, AddrMode::ZP0, 3),
    Inst(OpCode::DEY, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::TXA, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::STY, AddrMode::ABS, 4),
    Inst(OpCode::STA, AddrMode::ABS, 4),
    Inst(OpCode::STX, AddrMode::ABS, 4),
    Inst(OpCode::AXS, AddrMode::ABS, 4),
    Inst(OpCode::BCC, AddrMode::REL, 2),
    Inst(OpCode::STA, AddrMode::IZY, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::AXA, AddrMode::IZY, 6),
    Inst(OpCode::STY, AddrMode::ZPX, 4),
    Inst(OpCode::STA, AddrMode::ZPX, 4),
    Inst(OpCode::STX, AddrMode::ZPY, 4),
    Inst(OpCode::AXS, AddrMode::ZPY, 4),
    Inst(OpCode::TYA, AddrMode::IMP, 2),
    Inst(OpCode::STA, AddrMode::ABY, 5),
    Inst(OpCode::TXS, AddrMode::IMP, 2),
    Inst(OpCode::AXA, AddrMode::ABY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::STA, AddrMode::ABX, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::LDY, AddrMode::IMM, 2),
    Inst(OpCode::LDA, AddrMode::IZX, 6),
    Inst(OpCode::LDX, AddrMode::IMM, 2),
    Inst(OpCode::LAX, AddrMode::IZX, 6),
    Inst(OpCode::LDY, AddrMode::ZP0, 3),
    Inst(OpCode::LDA, AddrMode::ZP0, 3),
    Inst(OpCode::LDX, AddrMode::ZP0, 3),
    Inst(OpCode::LAX, AddrMode::ZP0, 3),
    Inst(OpCode::TAY, AddrMode::IMP, 2),
    Inst(OpCode::LDA, AddrMode::IMM, 2),
    Inst(OpCode::TAX, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::LDY, AddrMode::ABS, 4),
    Inst(OpCode::LDA, AddrMode::ABS, 4),
    Inst(OpCode::LDX, AddrMode::ABS, 4),
    Inst(OpCode::LAX, AddrMode::ABS, 4),
    Inst(OpCode::BCS, AddrMode::REL, 2),
    Inst(OpCode::LDA, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::LAX, AddrMode::IZY, 5),
    Inst(OpCode::LDY, AddrMode::ZPX, 4),
    Inst(OpCode::LDA, AddrMode::ZPX, 4),
    Inst(OpCode::LDX, AddrMode::ZPY, 4),
    Inst(OpCode::LAX, AddrMode::ZPY, 4),
    Inst(OpCode::CLV, AddrMode::IMP, 2),
    Inst(OpCode::LDA, AddrMode::ABY, 4),
    Inst(OpCode::TSX, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::LDY, AddrMode::ABX, 4),
    Inst(OpCode::LDA, AddrMode::ABX, 4),
    Inst(OpCode::LDX, AddrMode::ABY, 4),
    Inst(OpCode::LAX, AddrMode::ABY, 4),
    Inst(OpCode::CPY, AddrMode::IMM, 2),
    Inst(OpCode::CMP, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::DCM, AddrMode::IZX, 8),
    Inst(OpCode::CPY, AddrMode::ZP0, 3),
    Inst(OpCode::CMP, AddrMode::ZP0, 3),
    Inst(OpCode::DEC, AddrMode::ZP0, 5),
    Inst(OpCode::DCM, AddrMode::ZP0, 5),
    Inst(OpCode::INY, AddrMode::IMP, 2),
    Inst(OpCode::CMP, AddrMode::IMM, 2),
    Inst(OpCode::DEX, AddrMode::IMP, 2),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::CPY, AddrMode::ABS, 4),
    Inst(OpCode::CMP, AddrMode::ABS, 4),
    Inst(OpCode::DEC, AddrMode::ABS, 6),
    Inst(OpCode::DCM, AddrMode::ABS, 6),
    Inst(OpCode::BNE, AddrMode::REL, 2),
    Inst(OpCode::CMP, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::DCM, AddrMode::IZY, 8),
    Inst(OpCode::NII, AddrMode::ZPX, 4),
    Inst(OpCode::CMP, AddrMode::ZPX, 4),
    Inst(OpCode::DEC, AddrMode::ZPX, 6),
    Inst(OpCode::DCM, AddrMode::ZPX, 6),
    Inst(OpCode::CLD, AddrMode::IMP, 2),
    Inst(OpCode::CMP, AddrMode::ABY, 4),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::DCM, AddrMode::ABY, 7),
    Inst(OpCode::NII, AddrMode::ABX, 4),
    Inst(OpCode::CMP, AddrMode::ABX, 4),
    Inst(OpCode::DEC, AddrMode::ABX, 7),
    Inst(OpCode::DCM, AddrMode::ABX, 7),
    Inst(OpCode::CPX, AddrMode::IMM, 2),
    Inst(OpCode::SBC, AddrMode::IZX, 6),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::INS, AddrMode::IZX, 8),
    Inst(OpCode::CPX, AddrMode::ZP0, 3),
    Inst(OpCode::SBC, AddrMode::ZP0, 3),
    Inst(OpCode::INC, AddrMode::ZP0, 5),
    Inst(OpCode::INS, AddrMode::ZP0, 5),
    Inst(OpCode::INX, AddrMode::IMP, 2),
    Inst(OpCode::SBC, AddrMode::IMM, 2),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::SBC, AddrMode::IMM, 2),
    Inst(OpCode::CPX, AddrMode::ABS, 4),
    Inst(OpCode::SBC, AddrMode::ABS, 4),
    Inst(OpCode::INC, AddrMode::ABS, 6),
    Inst(OpCode::INS, AddrMode::ABS, 6),
    Inst(OpCode::BEQ, AddrMode::REL, 2),
    Inst(OpCode::SBC, AddrMode::IZY, 5),
    Inst(OpCode::XXX, AddrMode::IMP, 2),
    Inst(OpCode::INS, AddrMode::IZY, 8),
    Inst(OpCode::NII, AddrMode::ZPX, 4),
    Inst(OpCode::SBC, AddrMode::ZPX, 4),
    Inst(OpCode::INC, AddrMode::ZPX, 6),
    Inst(OpCode::INS, AddrMode::ZPX, 6),
    Inst(OpCode::SED, AddrMode::IMP, 2),
    Inst(OpCode::SBC, AddrMode::ABY, 4),
    Inst(OpCode::NII, AddrMode::IMP, 2),
    Inst(OpCode::INS, AddrMode::ABY, 7),
    Inst(OpCode::NII, AddrMode::ABX, 4),
    Inst(OpCode::SBC, AddrMode::ABX, 4),
    Inst(OpCode::INC, AddrMode::ABX, 7),
    Inst(OpCode::INS, AddrMode::ABX, 7),
];

struct Flag {
    c: bool, // Carry
    z: bool, // Zero
    i: bool, // Interrutps Disable
    b: bool,
    o: bool, // Overflow
    n: bool, // Nagative
}

impl Flag {
    fn new() -> Self {
        Self {
            c: false,
            z: false,
            i: false,
            b: false,
            o: false,
            n: false,
        }
    }
}

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    p: Flag,

    // Timing
    cycles: u64,
    clocks: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0xFFFC,
            p: Flag::new(),
            cycles: 0,
            clocks: 0,
        }
    }

    pub fn reset(&mut self, bus: &mut impl CPUBus) {
        self.pc = self.reset_vector(bus);

        self.sp -= 3;
        self.sp &= 0xFF;
        self.p.i = true;
    }

    fn reset_vector(&mut self, bus: &mut impl CPUBus) -> u16 {
        bus.cpu_read_u16(0xFFFC)
    }

    fn set_zn(&mut self, val: u8) {
        self.p.z = val == 0;
        self.p.n = (val & 0x80) != 0;
    }

    fn fetch_byte(&mut self, bus: &mut impl CPUBus) -> u8 {
        let data = bus.cpu_read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        data
    }

    fn fetch_u16(&mut self, bus: &mut impl CPUBus) -> u16 {
        let lo = self.fetch_byte(bus) as u16;
        let hi = self.fetch_byte(bus) as u16;
        (hi << 8) | lo
    }

    fn read_u16_zero_page(&mut self, addr: u8, bus: &mut impl CPUBus) -> u16 {
        let lo = bus.cpu_read(addr as u16) as u16;
        let hi = bus.cpu_read(addr.wrapping_add(1) as u16) as u16;
        (hi << 8) | lo
    }

    fn read_u16_indirect(&mut self, addr: u16, bus: &mut impl CPUBus) -> u16 {
        let lo = bus.cpu_read(addr) as u16;
        // Emulate the 6502 indirect JMP page-wrap bug.
        let hi_addr = (addr & 0xFF00) | (addr.wrapping_add(1) & 0x00FF);
        let hi = bus.cpu_read(hi_addr) as u16;
        (hi << 8) | lo
    }

    fn resolve_operand_addr(&mut self, mode: AddrMode, bus: &mut impl CPUBus) -> Option<u16> {
        match mode {
            AddrMode::IMM => Some(self.imm(bus)),
            AddrMode::IMP | AddrMode::JAM => None,
            AddrMode::ZP0 => Some(self.zp0(bus)),
            AddrMode::ZPX => Some(self.zpx(bus)),
            AddrMode::ZPY => Some(self.zpy(bus)),
            AddrMode::REL => Some(self.rel(bus)),
            AddrMode::ABS => Some(self.abs(bus)),
            AddrMode::ABX => Some(self.abx(bus)),
            AddrMode::ABY => Some(self.aby(bus)),
            AddrMode::IND => Some(self.ind(bus)),
            AddrMode::IZX => Some(self.izx(bus)),
            AddrMode::IZY => Some(self.izy(bus)),
        }
    }

    fn require_operand_addr(&mut self, mode: AddrMode, bus: &mut impl CPUBus) -> u16 {
        self.resolve_operand_addr(mode, bus)
            .expect("Instruction requires an operand address")
    }

    // fn fetch_operand(&mut self, addr: u16, bus: &mut impl CPUBus) -> u8 {
    //     self.resolve_operand_addr(mode, bus)
    //         .map(|addr| bus.cpu_read(addr))
    //         .unwrap_or(0)
    // }

    pub fn exe_inst(&mut self, inst_byte: u8, bus: &mut impl CPUBus) {
        let inst = INST_SET[inst_byte as usize];
        match inst.0 {
            OpCode::NOP => {
                let _ = self.resolve_operand_addr(inst.1, bus);
            } // Do nothing.
            OpCode::ORA => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.ora(addr, bus);
            }
            OpCode::AND => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.and(addr, bus);
            }
            OpCode::EOR => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.eor(addr, bus);
            }
            OpCode::LDA => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.lda(addr, bus);
            }
            OpCode::LDX => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.ldx(addr, bus);
            }
            OpCode::LDY => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.ldy(addr, bus);
            }
            OpCode::STA => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.sta(addr, bus);
            }
            OpCode::STX => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.stx(addr, bus);
            }
            OpCode::STY => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.sty(addr, bus);
            }
            OpCode::TAX => self.tax(),
            OpCode::TXA => self.txa(),
            OpCode::TAY => self.tay(),
            OpCode::TYA => self.tya(),
            OpCode::INX => self.inx(),
            OpCode::INY => self.iny(),
            OpCode::DEX => self.dex(),
            OpCode::DEY => self.dey(),
            OpCode::JMP => {
                let addr = self.require_operand_addr(inst.1, bus);
                self.jmp(addr);
            }
            _ => {
                let _ = self.resolve_operand_addr(inst.1, bus);
            } // Illegal opcode, do nothing
        }
        self.cycles += inst.2 as u64;
    }

    pub fn cpu_step(&mut self, bus: &mut impl CPUBus) {
        let inst_byte = bus.cpu_read(self.pc);
        self.pc += 1;
        self.clocks += 1;
        self.exe_inst(inst_byte, bus);
    }

    // === Addressing mode implementations ===
    // fn imp(&mut self, _bus: &mut impl CPUBus) -> u16 {
    //     0 // Implied addressing mode doesn't use an address
    // }

    fn imm(&mut self, _bus: &mut impl CPUBus) -> u16 {
        let addr = self.pc;
        self.pc = self.pc.wrapping_add(1);
        addr
    }

    fn zp0(&mut self, bus: &mut impl CPUBus) -> u16 {
        self.fetch_byte(bus) as u16
    }

    fn zpx(&mut self, bus: &mut impl CPUBus) -> u16 {
        self.fetch_byte(bus).wrapping_add(self.x) as u16
    }

    fn zpy(&mut self, bus: &mut impl CPUBus) -> u16 {
        self.fetch_byte(bus).wrapping_add(self.y) as u16
    }

    fn rel(&mut self, bus: &mut impl CPUBus) -> u16 {
        let offset = self.fetch_byte(bus) as i8;
        self.pc.wrapping_add_signed(offset as i16)
    }

    fn abs(&mut self, bus: &mut impl CPUBus) -> u16 {
        self.fetch_u16(bus)
    }

    fn abx(&mut self, bus: &mut impl CPUBus) -> u16 {
        self.fetch_u16(bus).wrapping_add(self.x as u16)
    }

    fn aby(&mut self, bus: &mut impl CPUBus) -> u16 {
        self.fetch_u16(bus).wrapping_add(self.y as u16)
    }

    fn ind(&mut self, bus: &mut impl CPUBus) -> u16 {
        let ptr = self.fetch_u16(bus);
        self.read_u16_indirect(ptr, bus)
    }

    fn izx(&mut self, bus: &mut impl CPUBus) -> u16 {
        let ptr = self.fetch_byte(bus).wrapping_add(self.x);
        self.read_u16_zero_page(ptr, bus)
    }

    fn izy(&mut self, bus: &mut impl CPUBus) -> u16 {
        let ptr = self.fetch_byte(bus);

        self.read_u16_zero_page(ptr, bus)
            .wrapping_add(self.y as u16)
    }

    // === Instruction implementations ===
    fn ora(&mut self, addr: u16, bus: &mut impl CPUBus) {
        self.a |= bus.cpu_read(addr);
        self.set_zn(self.a);
    }

    fn and(&mut self, addr: u16, bus: &mut impl CPUBus) {
        self.a &= bus.cpu_read(addr);
        self.set_zn(self.a);
    }

    fn eor(&mut self, addr: u16, bus: &mut impl CPUBus) {
        self.a ^= bus.cpu_read(addr);
        self.set_zn(self.a);
    }

    fn lda(&mut self, addr: u16, bus: &mut impl CPUBus) {
        self.a = bus.cpu_read(addr);
        self.set_zn(self.a);
    }

    fn ldx(&mut self, addr: u16, bus: &mut impl CPUBus) {
        self.x = bus.cpu_read(addr);
        self.set_zn(self.x);
    }

    fn ldy(&mut self, addr: u16, bus: &mut impl CPUBus) {
        self.y = bus.cpu_read(addr);
        self.set_zn(self.y);
    }

    fn sta(&mut self, addr: u16, bus: &mut impl CPUBus) {
        bus.cpu_write(addr, self.a);
    }

    fn stx(&mut self, addr: u16, bus: &mut impl CPUBus) {
        bus.cpu_write(addr, self.x);
    }

    fn sty(&mut self, addr: u16, bus: &mut impl CPUBus) {
        bus.cpu_write(addr, self.y);
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.set_zn(self.x);
    }

    fn txa(&mut self) {
        self.a = self.x;
        self.set_zn(self.a);
    }

    fn tay(&mut self) {
        self.y = self.a;
        self.set_zn(self.y);
    }

    fn tya(&mut self) {
        self.a = self.y;
        self.set_zn(self.a);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_zn(self.x);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.set_zn(self.x);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_zn(self.y);
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_zn(self.y);
    }

    fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBus {
        mem: [u8; 0x10000],
    }

    impl TestBus {
        fn new() -> Self {
            Self { mem: [0; 0x10000] }
        }

        fn write_u16(&mut self, addr: u16, value: u16) {
            self.cpu_write(addr, value as u8);
            self.cpu_write(addr.wrapping_add(1), (value >> 8) as u8);
        }
    }

    impl CPUBus for TestBus {
        fn cpu_read(&mut self, addr: u16) -> u8 {
            self.mem[addr as usize]
        }

        fn cpu_write(&mut self, addr: u16, data: u8) {
            self.mem[addr as usize] = data;
        }
    }

    mod reset {
        use super::*;

        #[test]
        fn sets_pc_stack_pointer_and_interrupt_disable_from_reset_vector() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            bus.write_u16(0xFFFC, 0x1234);

            cpu.reset(&mut bus);

            assert_eq!(cpu.pc, 0x1234);
            assert_eq!(cpu.sp, 0xFA);
            assert!(cpu.p.i, "interrupt disable flag should be set after reset");
        }
    }

    mod addressing {
        use super::*;

        #[test]
        fn rel_returns_signed_target_and_advances_pc() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x2000;
            bus.cpu_write(0x2000, 0xFE);

            let addr = cpu.resolve_operand_addr(AddrMode::REL, &mut bus);

            assert_eq!(addr, Some(0x1FFF));
            assert_eq!(cpu.pc, 0x2001);
        }

        #[test]
        fn izx_wraps_zero_page_pointer_before_reading_effective_address() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x3000;
            cpu.x = 0x20;
            bus.cpu_write(0x3000, 0xF0);
            bus.cpu_write(0x0010, 0xCD);
            bus.cpu_write(0x0011, 0xAB);

            let addr = cpu.resolve_operand_addr(AddrMode::IZX, &mut bus);

            assert_eq!(addr, Some(0xABCD));
            assert_eq!(cpu.pc, 0x3001);
        }

        #[test]
        fn ind_emulates_6502_page_wrap_bug() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x4000;
            bus.write_u16(0x4000, 0x12FF);
            bus.cpu_write(0x12FF, 0x78);
            bus.cpu_write(0x1200, 0x56);
            bus.cpu_write(0x1300, 0x99);

            let addr = cpu.resolve_operand_addr(AddrMode::IND, &mut bus);

            assert_eq!(addr, Some(0x5678));
            assert_eq!(cpu.pc, 0x4002);
        }
    }

    mod execution {
        use super::*;

        #[test]
        fn cpu_step_executes_ora_immediate_and_updates_flags_and_timing() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            cpu.a = 0x80;
            bus.cpu_write(0x0200, 0x09);
            bus.cpu_write(0x0201, 0x01);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.a, 0x81);
            assert_eq!(cpu.pc, 0x0202);
            assert!(!cpu.p.z, "accumulator should not be zero");
            assert!(cpu.p.n, "bit 7 should set the negative flag");
            assert_eq!(cpu.cycles, 2);
            assert_eq!(cpu.clocks, 1);
        }

        #[test]
        fn cpu_step_executes_lda_immediate_and_sets_zero_flag() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            bus.cpu_write(0x0200, 0xA9);
            bus.cpu_write(0x0201, 0x00);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.a, 0x00);
            assert!(cpu.p.z, "loading zero should set the zero flag");
            assert!(!cpu.p.n, "loading zero should clear the negative flag");
            assert_eq!(cpu.pc, 0x0202);
        }

        #[test]
        fn cpu_step_executes_sta_zero_page_and_writes_accumulator_to_bus() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            cpu.a = 0x5A;
            bus.cpu_write(0x0200, 0x85);
            bus.cpu_write(0x0201, 0x10);

            cpu.cpu_step(&mut bus);

            assert_eq!(bus.cpu_read(0x0010), 0x5A);
            assert_eq!(cpu.pc, 0x0202);
        }

        #[test]
        fn cpu_step_executes_tax_and_updates_zero_and_negative_flags() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            cpu.a = 0x80;
            bus.cpu_write(0x0200, 0xAA);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.x, 0x80);
            assert!(!cpu.p.z, "copied value is not zero");
            assert!(cpu.p.n, "copied value has bit 7 set");
            assert_eq!(cpu.pc, 0x0201);
        }

        #[test]
        fn cpu_step_executes_jmp_absolute() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            bus.cpu_write(0x0200, 0x4C);
            bus.write_u16(0x0201, 0x3456);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.pc, 0x3456);
            assert_eq!(cpu.cycles, 3);
            assert_eq!(cpu.clocks, 1);
        }

        #[test]
        fn cpu_step_executes_nop_zero_page_and_consumes_operand_byte() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            cpu.a = 0x3C;
            cpu.x = 0x12;
            cpu.y = 0x34;
            bus.cpu_write(0x0200, 0x04);
            bus.cpu_write(0x0201, 0x99);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.pc, 0x0202);
            assert_eq!(cpu.a, 0x3C);
            assert_eq!(cpu.x, 0x12);
            assert_eq!(cpu.y, 0x34);
            assert_eq!(cpu.cycles, 3);
            assert_eq!(cpu.clocks, 1);
        }

        #[test]
        fn cpu_step_unimplemented_opcode_with_absolute_mode_still_advances_pc() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            cpu.a = 0x11;
            cpu.x = 0x22;
            cpu.y = 0x33;
            bus.cpu_write(0x0200, 0x0E);
            bus.write_u16(0x0201, 0x4567);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.pc, 0x0203);
            assert_eq!(cpu.a, 0x11);
            assert_eq!(cpu.x, 0x22);
            assert_eq!(cpu.y, 0x33);
            assert_eq!(cpu.cycles, 6);
            assert_eq!(cpu.clocks, 1);
        }

        #[test]
        fn cpu_step_executes_jmp_indirect_using_wrapped_high_byte() {
            let mut cpu = CPU::new();
            let mut bus = TestBus::new();
            cpu.pc = 0x0200;
            bus.cpu_write(0x0200, 0x6C);
            bus.write_u16(0x0201, 0x12FF);
            bus.cpu_write(0x12FF, 0x34);
            bus.cpu_write(0x1200, 0x12);
            bus.cpu_write(0x1300, 0x99);

            cpu.cpu_step(&mut bus);

            assert_eq!(cpu.pc, 0x1234);
            assert_eq!(cpu.cycles, 5);
            assert_eq!(cpu.clocks, 1);
        }
    }
}
