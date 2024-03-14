#[repr(u8)]
pub enum CgbFlag {
    // Game supports CGB functions, but works on old gameboys also.
    OldGameboys = 0x80,
    // Game works on CGB only (physically the same as 80h).
    CgbOnly = 0xC0,
}

// Values with Bit 7 set, and either Bit 2 or 3 set, will switch the gameboy into a special
// non-CGB-mode with uninitialized palettes.
// Purpose unknown, eventually this has been supposed to be used to colorize monochrome
// games that include fixed palette data at a special location in ROM.
