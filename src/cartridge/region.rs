#[repr(u8)]
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
