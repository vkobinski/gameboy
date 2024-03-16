use serde::Deserialize;
use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Deserialize, Debug, FromPrimitive)]
pub enum CgbFlag {
    // Game supports CGB functions, but works on old gameboys also.
    OldGameboys = 0x80,
    // Game works on CGB only (physically the same as 80h).
    CgbOnly = 0xC0,
    None = 0x00,
}

// Values with Bit 7 set, and either Bit 2 or 3 set, will switch the gameboy into a special
// non-CGB-mode with uninitialized palettes.
// Purpose unknown, eventually this has been supposed to be used to colorize monochrome
// games that include fixed palette data at a special location in ROM.
//

impl TryFrom<u8> for CgbFlag {

    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {

        let el = num::FromPrimitive::from_u8(value);

        match el {
            Some(val) => Ok(val),
            None => Err("Could not parse CgbFlag"),
        }

    }

}
