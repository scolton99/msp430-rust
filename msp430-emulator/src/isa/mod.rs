mod format;

pub enum AddressingMode {
    Register,
    Indexed,
    Symbolic,
    Absolute,
    IndirectRegister,
    IndirectAutoincrement,
    Immediate
}

pub enum InstructionWidth {
    Byte,
    Word,
    AddressWord
}
