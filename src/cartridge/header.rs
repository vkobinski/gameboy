use super::{c_type::CartridgeType, cgb::CgbFlag, licensee::Licensee, rom_size::ByteRomSize};

#[derive(Debug)]
pub struct Header {
    pub entry_point: usize,
    pub nintendo_logo: [u8; 0x30],
    pub title: [char; 0xF],
    pub manufacturer_code: [char; 0x3],
    //pub cgb_flag: CgbFlag,
    //pub licensee_code: Licensee,
    //pub c_type: CartridgeType,
    //pub rom_size: ByteRomSize,

}

impl Header {

    pub fn from_bytes(bytes: &[u8]) -> Option<Header> {

        if bytes.len() < 0x50 {
            return None;
        }

        let mut entry_buf: [u8; 8] = [0; 8];

        for (index, byte) in bytes[0x100..0x104].into_iter().enumerate() {

            entry_buf[index] = *byte;
        }

        let entry_point = usize::from_le_bytes(entry_buf);

        let nintendo_logo = bytes[0x104..0x134].try_into().ok()?;

        let title = bytes[0x134..0x143]
            .iter()
            .map(|&b| char::from(b))
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        let manufacturer_code = bytes[0x13F..0x142]
            .iter()
            .map(|&b| char::from(b))
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        //let cgb_flag = CgbFlag::try_from(bytes[0x143]).ok()?;
        //let licensee_code = Licensee::try_from(bytes[0x144..0x146].try_into().ok()?).ok()?;
        //let c_type = CartridgeType::try_from(bytes[0x147]).ok()?;
        //let rom_size = ByteRomSize::from_code(bytes[0x148]).ok_or(())?;

        Some(Header {
            entry_point,
            nintendo_logo,
            title,
            manufacturer_code,
            //cgb_flag,
            //licensee_code,
            //c_type,
            //rom_size,
        })
    }

}
