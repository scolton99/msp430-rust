struct ExtensionWord {
    zero_carry: bool,
    src_msn:    u8,
    dst_msn:    u8,
    repeat:     u8,
    is_legacy:  bool,
    is_use_reg: bool
}

impl ExtensionWord {
    pub fn from(word: u16) -> Self {
        ExtensionWord {
            zero_carry: ((word >> 8) as u8) & 0b1 == 1,
            is_use_reg: ((word >> 7) & 0b1) as u8 == 1,
            repeat:     (word & 0xF) as u8,
            src_msn:    ((word >> 7) & 0xF) as u8,
            dst_msn:    (word & 0xF) as u8,
            is_legacy:  ((word >> 6) & 0b1) as u8 == 1
        }
    }
}

enum Instruction {
    FormatI {
        src_reg:  u8,
        src_addr: u8,
        dst_addr: u8,
        dst_reg:  u8,
        ext_word: Option<ExtensionWord>
    },
    FormatII {
        is_byte: u8
    },
    FormatAddress,
    FormatCallA,
    FormatJump,
    FormatRXXM,
    FormatPushMPopM
}

impl Instruction {

}