use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Region {
    Japanese = 0x00,
    NonJapanese = 0x01,
}

impl ToString for Region {

    fn to_string(&self) -> String {
        match self {
            Region::Japanese => "Japanese".to_string(),
            Region::NonJapanese => "Non-Japanese".to_string(),
        }
    }
}


impl TryFrom<u8> for Region {

    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {

        let el = num::FromPrimitive::from_u8(value);

        match el {
            Some(val) => Ok(val),
            None => Err("Could not parse DestinationCode"),
        }

    }

}
