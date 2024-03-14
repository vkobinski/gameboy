#[repr(u8)]
pub enum SgbFlag {
    // No SGB functions (Normal Gameboy or CGB only game)
    NoSgb = 0x0,
    // Game supports SGB functions
    HasSgb = 0x03,
}

// !! The SGB disables its SGB functions if this byte is set to another value than 03h.
