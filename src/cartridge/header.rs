use num::range;

use super::{c_type::CartridgeType, cgb::CgbFlag, licensee::Licensee, old_licensee::OldLicensee, region::Region, rom_size::ByteRomSize, sgb::SgbFlag};

#[derive(Debug)]
pub struct Header {
    pub nintendo_logo: [u8; 0x30],
    pub title: [char; 0xF],
    pub manufacturer_code: [char; 0x3],
    pub cgb_flag: CgbFlag,
    pub licensee_code: Licensee,
    pub sgb_flag: SgbFlag,
    pub c_type: CartridgeType,
    pub rom_size: ByteRomSize,
    pub old_licensee: OldLicensee,
    pub region: Region,
    pub global_checksum: u16,
}

impl Header {

    pub fn header_checksum_validation(bytes: &[u8], expected: u8) {

        let start: usize = 0x0134;

        let mut checksum: u8 = 0;

        for address in range(start, 0x014C + 1) {
            checksum = checksum.wrapping_sub(bytes[address]) - 1;
        }

        if checksum != expected {
            panic!("Failed header checksum");
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Header> {

        if bytes.len() < 0x50 {
            return None;
        }

        let nintendo_logo = bytes[0x104..0x134].try_into().ok()?;

        let manufacturer_code = bytes[0x13F..0x142]
            .iter()
            .map(|&b| char::from(b))
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        let title = bytes[0x134..0x143]
            .iter()
            .map(|&b| char::from(b))
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        let mut licensee_buf : [u8; 2] = [0;2];
        licensee_buf.copy_from_slice(&bytes[0x144..0x146]);

        let rom_size = ByteRomSize::from_code(bytes[0x148]).ok_or(()).unwrap();
        let licensee_code = Licensee::try_from(licensee_buf).unwrap();
        let cgb_flag = CgbFlag::try_from(bytes[0x143]).unwrap();
        let c_type = CartridgeType::try_from(bytes[0x147]).ok()?;
        let old_licensee = OldLicensee::try_from(bytes[0x14B]).unwrap();
        let region = Region::try_from(bytes[0x14A]).unwrap();
        let sgb_flag = SgbFlag::try_from(bytes[0x146]).unwrap();

        let mut global_buf : [u8; 2] = [0;2];
        global_buf.copy_from_slice(&bytes[0x14E..0x150]);

        let header_checksum = bytes[0x14D];
        let global_checksum = u16::from_be_bytes(global_buf);

        Self::header_checksum_validation(bytes, header_checksum);

        Some(Header {
            nintendo_logo,
            title,
            manufacturer_code,
            cgb_flag,
            licensee_code,
            c_type,
            rom_size,
            old_licensee,
            region,
            sgb_flag,
            global_checksum
        })
    }

}
