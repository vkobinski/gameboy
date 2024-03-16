use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum SgbFlag {
    // No SGB functions (Normal Gameboy or CGB only game)
    NoSgb = 0x0,
    // Game supports SGB functions
    HasSgb = 0x03,
}

// !! The SGB disables its SGB functions if this byte is set to another value than 03h.

impl TryFrom<u8> for SgbFlag {

    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {

        let el = num::FromPrimitive::from_u8(value);

        match el {
            Some(val) => Ok(val),
            None => Err("Could not parse SgbFlag"),
        }

    }

}
