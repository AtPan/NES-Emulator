pub type Instruction = (Opcode, AugmentOpcodeInfo);

#[derive(Copy, Clone, Debug)]
pub enum AugmentOpcodeInfo {
    Implied,
    Accumulator,
    Immediate(u8), /* An Immediate 8-bit value */
    MemoryByte(u16), /* A byte from memory at the indicated address */
    MemoryWord(u16), /* A word from memory at the indicated address */
    Address(u16),
}

#[derive(Copy, Clone)]
pub enum MemAddressMode {
    Accumulator,        /* Operand is accumulator register (implied 8-bit instruction) */
    Absolute,           /* Operand is given 16-bit address */
    AbsoluteIndexedX,   /* Operand is given 16-bit address, incremented by x register with carry */
    AbsoluteIndexedY,   /* Operand is given 16-bit address, incremented by y register with carry */
    Immediate,          /* Operand is given 8-bit value */
    Implied,            /* Operand is implied by the instruction */
    Indirect,           /* Operand is given 16-bit address; effective address is contents of word at address */
    IndirectIndexedX,   /* Operand is given 16-bit address, incremented by x register w/o carry; effective address is contents of word at address */
    IndirectIndexedY,   /* Operand is given 16-bit address, incremented by y register w/o carry; effective address is contents of word at address */
    Relative,           /* Branch target is PC offset by given signed 8-bit value */
    ZeroPage,           /* Operand is address in the zero page by given 8-bit value */
    ZeroPageIndexedX,   /* Operand is address in the zero page, incremented by x register w/o carry */
    ZeroPageIndexedY,   /* Operand is address in the zero page, incremented by y register w/o carry */
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    ADC,    /* Add with carry */
    AND,    /* And with accumulator */
    ASL,    /* Arithmetic shift left */
    BCC,    /* Branch on carry clear */
    BCS,    /* Branch on carry set */
    BEQ,    /* Branch on equal (zero set) */
    BIT,    /* Bit test */
    BMI,    /* Branch on minus (negative set) */
    BNE,    /* Branch on not equal (zero clear) */
    BPL,    /* Branch on plus (negative clear) */
    BRK,    /* Break / interrupt */
    BVC,    /* Branch on overflow clear */
    BVS,    /* Branch on overflow set */
    CLC,    /* Clear carry */
    CLD,    /* Clear decimal */
    CLI,    /* Clear interrupt disable */
    CLV,    /* Clear overflow */
    CMP,    /* Compare with accumulator */
    CPX,    /* Compare with x */
    CPY,    /* Compare with y */
    DEC,    /* Decrement */
    DEX,    /* Decrement x */
    DEY,    /* Decrement y */
    EOR,    /* XOR with accumulator */
    INC,    /* Increment */
    INX,    /* Increment x */
    INY,    /* Increment y */
    JMP,    /* Jump */
    JSR,    /* Jump subroutine */
    LDA,    /* Load accumulator */
    LDX,    /* Load x */
    LDY,    /* Load y */
    LSR,    /* Logical shift right */
    NOP,    /* No operation */
    ORA,    /* Or with accumulator */
    PHA,    /* Push accumulator */
    PHP,    /* Push processor status */
    PLA,    /* Pull accumulator */
    PLP,    /* Pull processor status */
    ROL,    /* Rotate left */
    ROR,    /* Rotate right */
    RTI,    /* Return from interrupt */
    RTS,    /* Return from subroutine */
    SBC,    /* Subtract with carry */
    SEC,    /* Set carry */
    SED,    /* Set decimal */
    SEI,    /* Set interrupt disable */
    STA,    /* Store accumulator */
    STX,    /* Store x */
    STY,    /* Store y */
    TAX,    /* Transfer accumulator to x */
    TAY,    /* Transfer accumulator to y */
    TSX,    /* Transfer stack pointer to x */
    TXA,    /* Transfer x to accumulator */
    TXS,    /* Transfer x to stack pointer */
    TYA,    /* Transfer y to accumulator */
}

pub static OPCODES: [Option<(Opcode, MemAddressMode)>; 256] = [
    //0x00
    Some((Opcode::BRK, MemAddressMode::Implied)),
    Some((Opcode::ORA, MemAddressMode::IndirectIndexedX)),
    None, None, None,
    Some((Opcode::ORA, MemAddressMode::ZeroPage)),
    Some((Opcode::ASL, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::PHP, MemAddressMode::Implied)),
    Some((Opcode::ORA, MemAddressMode::Immediate)),
    Some((Opcode::ASL, MemAddressMode::Accumulator)),
    None, None,
    Some((Opcode::ORA, MemAddressMode::Absolute)),
    Some((Opcode::ASL, MemAddressMode::Absolute)),
    None,

    //0x10
    Some((Opcode::BPL, MemAddressMode::Relative)),
    Some((Opcode::ORA, MemAddressMode::IndirectIndexedY)),
    None, None, None,
    Some((Opcode::ORA, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::ASL, MemAddressMode::ZeroPageIndexedX)),
    None,
    Some((Opcode::CLC, MemAddressMode::Implied)),
    Some((Opcode::ORA, MemAddressMode::AbsoluteIndexedY)),
    None, None, None,
    Some((Opcode::ORA, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::ASL, MemAddressMode::AbsoluteIndexedX)),
    None,

    //0x20
    Some((Opcode::JSR, MemAddressMode::Absolute)),
    Some((Opcode::AND, MemAddressMode::IndirectIndexedX)),
    None, None,
    Some((Opcode::BIT, MemAddressMode::ZeroPage)),
    Some((Opcode::AND, MemAddressMode::ZeroPage)),
    Some((Opcode::ROL, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::PLP, MemAddressMode::Implied)),
    Some((Opcode::AND, MemAddressMode::Immediate)),
    Some((Opcode::ROL, MemAddressMode::Accumulator)),
    None,
    Some((Opcode::BIT, MemAddressMode::Absolute)),
    Some((Opcode::AND, MemAddressMode::Absolute)),
    Some((Opcode::ROL, MemAddressMode::Absolute)),
    None,

    //0x30
    Some((Opcode::BMI, MemAddressMode::Relative)),
    Some((Opcode::AND, MemAddressMode::IndirectIndexedY)),
    None, None, None,
    Some((Opcode::AND, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::ROL, MemAddressMode::ZeroPageIndexedX)),
    None,
    Some((Opcode::SEC, MemAddressMode::Implied)),
    Some((Opcode::AND, MemAddressMode::AbsoluteIndexedY)),
    None, None, None,
    Some((Opcode::AND, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::ROL, MemAddressMode::AbsoluteIndexedX)),
    None,

    //0x40
    Some((Opcode::RTI, MemAddressMode::Implied)),
    Some((Opcode::EOR, MemAddressMode::IndirectIndexedX)),
    None, None, None,
    Some((Opcode::EOR, MemAddressMode::ZeroPage)),
    Some((Opcode::LSR, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::PHA, MemAddressMode::Implied)),
    Some((Opcode::EOR, MemAddressMode::Immediate)),
    Some((Opcode::LSR, MemAddressMode::Accumulator)),
    None,
    Some((Opcode::JMP, MemAddressMode::Absolute)),
    Some((Opcode::EOR, MemAddressMode::Absolute)),
    Some((Opcode::LSR, MemAddressMode::Absolute)),
    None,

    //0x50
    Some((Opcode::BVC, MemAddressMode::Relative)),
    Some((Opcode::EOR, MemAddressMode::IndirectIndexedY)),
    None, None, None,
    Some((Opcode::EOR, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::LSR, MemAddressMode::ZeroPageIndexedX)),
    None,
    Some((Opcode::CLI, MemAddressMode::Implied)),
    Some((Opcode::EOR, MemAddressMode::AbsoluteIndexedY)),
    None, None, None,
    Some((Opcode::EOR, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::LSR, MemAddressMode::AbsoluteIndexedX)),
    None,

    //0x60
    Some((Opcode::RTS, MemAddressMode::Implied)),
    Some((Opcode::ADC, MemAddressMode::IndirectIndexedX)),
    None, None, None,
    Some((Opcode::ADC, MemAddressMode::ZeroPage)),
    Some((Opcode::ROR, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::PLA, MemAddressMode::Implied)),
    Some((Opcode::ADC, MemAddressMode::Immediate)),
    Some((Opcode::ROR, MemAddressMode::Accumulator)),
    None,
    Some((Opcode::JMP, MemAddressMode::Indirect)),
    Some((Opcode::ADC, MemAddressMode::Absolute)),
    Some((Opcode::ROR, MemAddressMode::Absolute)),
    None,

    //0x70
    Some((Opcode::BVS, MemAddressMode::Relative)),
    Some((Opcode::ADC, MemAddressMode::IndirectIndexedY)),
    None, None, None,
    Some((Opcode::ADC, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::ROR, MemAddressMode::ZeroPageIndexedX)),
    None,
    Some((Opcode::SEI, MemAddressMode::Implied)),
    Some((Opcode::ADC, MemAddressMode::AbsoluteIndexedY)),
    None, None, None,
    Some((Opcode::ADC, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::ROR, MemAddressMode::AbsoluteIndexedX)),
    None,

    //0x80
    None,
    Some((Opcode::STA, MemAddressMode::IndirectIndexedX)),
    None, None,
    Some((Opcode::STY, MemAddressMode::ZeroPage)),
    Some((Opcode::STA, MemAddressMode::ZeroPage)),
    Some((Opcode::STX, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::DEY, MemAddressMode::Implied)),
    None,
    Some((Opcode::TXA, MemAddressMode::Implied)),
    None,
    Some((Opcode::STY, MemAddressMode::Absolute)),
    Some((Opcode::STA, MemAddressMode::Absolute)),
    Some((Opcode::STX, MemAddressMode::Absolute)),
    None,

    //0x90
    Some((Opcode::BCC, MemAddressMode::Relative)),
    Some((Opcode::STA, MemAddressMode::IndirectIndexedY)),
    None, None,
    Some((Opcode::STY, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::STA, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::STX, MemAddressMode::ZeroPageIndexedY)),
    None,
    Some((Opcode::TYA, MemAddressMode::Implied)),
    Some((Opcode::STA, MemAddressMode::AbsoluteIndexedY)),
    Some((Opcode::TXS, MemAddressMode::Implied)),
    None, None,
    Some((Opcode::STA, MemAddressMode::AbsoluteIndexedX)),
    None, None,

    //0xa0
    Some((Opcode::LDY, MemAddressMode::Immediate)),
    Some((Opcode::LDA, MemAddressMode::IndirectIndexedX)),
    Some((Opcode::LDX, MemAddressMode::Immediate)),
    None,
    Some((Opcode::LDY, MemAddressMode::ZeroPage)),
    Some((Opcode::LDA, MemAddressMode::ZeroPage)),
    Some((Opcode::LDX, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::TAY, MemAddressMode::Implied)),
    Some((Opcode::LDA, MemAddressMode::Immediate)),
    Some((Opcode::TAX, MemAddressMode::Implied)),
    None,
    Some((Opcode::LDY, MemAddressMode::Absolute)),
    Some((Opcode::LDA, MemAddressMode::Absolute)),
    Some((Opcode::LDX, MemAddressMode::Absolute)),
    None,

    //0xb0
    Some((Opcode::BCS, MemAddressMode::Relative)),
    Some((Opcode::LDA, MemAddressMode::IndirectIndexedY)),
    None, None,
    Some((Opcode::LDY, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::LDA, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::LDX, MemAddressMode::ZeroPageIndexedY)),
    None,
    Some((Opcode::CLV, MemAddressMode::Implied)),
    Some((Opcode::LDA, MemAddressMode::AbsoluteIndexedY)),
    Some((Opcode::TSX, MemAddressMode::Implied)),
    None,
    Some((Opcode::LDY, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::LDA, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::LDX, MemAddressMode::AbsoluteIndexedY)),
    None,

    //0xc0
    Some((Opcode::CPY, MemAddressMode::Immediate)),
    Some((Opcode::CMP, MemAddressMode::IndirectIndexedX)),
    None, None,
    Some((Opcode::CPY, MemAddressMode::ZeroPage)),
    Some((Opcode::CMP, MemAddressMode::ZeroPage)),
    Some((Opcode::DEC, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::INY, MemAddressMode::Implied)),
    Some((Opcode::CMP, MemAddressMode::Immediate)),
    Some((Opcode::DEX, MemAddressMode::Implied)),
    None,
    Some((Opcode::CPY, MemAddressMode::Absolute)),
    Some((Opcode::CMP, MemAddressMode::Absolute)),
    Some((Opcode::DEC, MemAddressMode::Absolute)),
    None,

    //0xd0
    Some((Opcode::BNE, MemAddressMode::Relative)),
    Some((Opcode::CMP, MemAddressMode::IndirectIndexedY)),
    None, None, None,
    Some((Opcode::CMP, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::DEC, MemAddressMode::ZeroPageIndexedX)),
    None,
    Some((Opcode::CLD, MemAddressMode::Implied)),
    Some((Opcode::CMP, MemAddressMode::AbsoluteIndexedY)),
    None, None, None,
    Some((Opcode::CMP, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::DEC, MemAddressMode::AbsoluteIndexedX)),
    None,

    //0xe0
    Some((Opcode::CPX, MemAddressMode::Immediate)),
    Some((Opcode::SBC, MemAddressMode::IndirectIndexedX)),
    None, None,
    Some((Opcode::CPX, MemAddressMode::ZeroPage)),
    Some((Opcode::SBC, MemAddressMode::ZeroPage)),
    Some((Opcode::INC, MemAddressMode::ZeroPage)),
    None,
    Some((Opcode::INX, MemAddressMode::Implied)),
    Some((Opcode::SBC, MemAddressMode::Immediate)),
    Some((Opcode::NOP, MemAddressMode::Implied)),
    None,
    Some((Opcode::CPX, MemAddressMode::Absolute)),
    Some((Opcode::SBC, MemAddressMode::Absolute)),
    Some((Opcode::INC, MemAddressMode::Absolute)),
    None,

    //0xf0
    Some((Opcode::BEQ, MemAddressMode::Relative)),
    Some((Opcode::SBC, MemAddressMode::IndirectIndexedY)),
    None, None, None,
    Some((Opcode::SBC, MemAddressMode::ZeroPageIndexedX)),
    Some((Opcode::INC, MemAddressMode::ZeroPageIndexedX)),
    None,
    Some((Opcode::SED, MemAddressMode::Implied)),
    Some((Opcode::SBC, MemAddressMode::AbsoluteIndexedY)),
    None, None, None,
    Some((Opcode::SBC, MemAddressMode::AbsoluteIndexedX)),
    Some((Opcode::INC, MemAddressMode::AbsoluteIndexedX)),
    None,
];
