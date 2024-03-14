use super::cgb::CgbFlag;

pub struct Header {
    pub entry_point: usize,
    pub nintendo_logo: [u8; 0x2F],
    pub title: [char; 0xF],
    pub manufacturer_code: [char; 0x3],
    pub cgb_flag: CgbFlag,

}
