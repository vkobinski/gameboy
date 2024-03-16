use serde::Deserialize;
use num_derive::FromPrimitive;


#[repr(u16)]
#[derive(Deserialize, Debug, FromPrimitive, Clone)]
pub enum Licensee {
    UseOldLicensee = 00,
    NintendoRD1 = 01,
    Capcom = 08,
    ElectronicArts = 13 | 69,
    HudsonSoft = 18,
    Bai = 19,
    Kss = 20,
    Pow = 22,
    PCM = 24,
    Sanx = 25,
    Kemco = 28,
    Seta = 29,
    Viacom = 30,
    Nintendo = 31,
    Bandai = 32,
    OceanAcclaim = 33 | 93,
    Konami = 34 | 54,
    Hector = 35,
    Taito = 37,
    Hudson = 38,
    Banpresto = 39,
    UbiSoft = 41,
    Atlus = 42,
    Malibu = 44,
    Angel = 46,
    BulletProof = 47,
    Irem = 49,
    Absolute = 50,
    Acclaim = 51,
    Activision = 52,
    AmericanSammy = 53,
    HiTechEntertainment = 55,
    LJN = 56,
    Matchbox = 57,
    Mattel = 58,
    MiltonBradley = 59,
    Titus = 60,
    Virgin = 61,
    LucasArts = 64,
    Ocean = 67,
    Infogrames = 70,
    Interplay = 71,
    Broderbund = 72,
    Sculptured = 73,
    Sci = 75,
    THQ = 78,
    Accolade = 79,
    Misawa = 80,
    Lozc = 83,
    TokumaShotenI = 86,
    TsukudaOri = 87,
    Chunsoft = 91,
    VideoSystem = 92,
    Varie = 95,
    Yonezawaspal = 96,
    Kaneko = 97,
    PackInSoft = 99,
    Yugioh = 0xA4
}

impl Into<&str> for Licensee {
    fn into(self) -> &'static str {
        match &self {
            Licensee::UseOldLicensee => "Use old licensee",
            Licensee::NintendoRD1 => "Nintendo R&D1",
            Licensee::Capcom => "Capcom",
            Licensee::ElectronicArts => "Electronic Arts",
            Licensee::HudsonSoft => "Hudson Soft",
            Licensee::Bai => "b-ai",
            Licensee::Kss => "kss",
            Licensee::Pow => "pow",
            Licensee::PCM => "PCM Complete",
            Licensee::Sanx => "san-x",
            Licensee::Kemco => "Kemco Japan",
            Licensee::Seta => "seta",
            Licensee::Viacom => "Viacom",
            Licensee::Nintendo => "Nintendo",
            Licensee::Bandai => "Bandai",
            Licensee::OceanAcclaim => "Ocean/Acclaim",
            Licensee::Konami => "Konami",
            Licensee::Hector => "Hector",
            Licensee::Taito => "Taito",
            Licensee::Hudson => "Hudson",
            Licensee::Banpresto => "Banpresto",
            Licensee::UbiSoft => "Ubi Soft",
            Licensee::Atlus => "Atlus",
            Licensee::Malibu => "Malibu",
            Licensee::Angel => "angel",
            Licensee::BulletProof => "Bullet-Proof",
            Licensee::Irem => "irem",
            Licensee::Absolute => "Absolute",
            Licensee::Acclaim => "Acclaim",
            Licensee::Activision => "Activision",
            Licensee::AmericanSammy => "American sammy",
            Licensee::HiTechEntertainment => "Hi tech entertainment",
            Licensee::LJN => "LJN",
            Licensee::Matchbox => "Matchbox",
            Licensee::Mattel => "Mattel",
            Licensee::MiltonBradley => "Milton Bradley",
            Licensee::Titus => "Titus",
            Licensee::Virgin => "Virgin",
            Licensee::LucasArts => "LucasArts",
            Licensee::Ocean => "Ocean",
            Licensee::Infogrames => "Infogrames",
            Licensee::Interplay => "Interplay",
            Licensee::Broderbund => "Broderbund",
            Licensee::Sculptured => "sculptured",
            Licensee::Sci => "sci",
            Licensee::THQ => "THQ",
            Licensee::Accolade => "Accolade",
            Licensee::Misawa => "misawa",
            Licensee::Lozc => "lozc",
            Licensee::TokumaShotenI => "tokuma shoten i*",
            Licensee::TsukudaOri => "tsukuda ori*",
            Licensee::Chunsoft => "Chunsoft",
            Licensee::VideoSystem => "Video system",
            Licensee::Varie => "Varie",
            Licensee::Yonezawaspal => "Yonezawa/s'pal",
            Licensee::Kaneko => "Kaneko",
            Licensee::PackInSoft => "Pack in soft",
            Licensee::Yugioh => "Konami (Yu-Gi-Oh!)",
        }
    }
}

 impl TryFrom<[u8; 2]> for Licensee {

     type Error = &'static str;

     fn try_from(value: [u8;2]) -> Result<Self, Self::Error> {

        let char_buf: [char; 2] = [value[0] as char, value[1] as char];

        if value[0] == 0 && value[1] == 0 {

            return Ok(Licensee::UseOldLicensee);

        }

        let val_res : [u8; 2] = [char_buf[0] as u8 - b'0', char_buf[1] as u8 - b'0'];

         let val_char = u16::from_be_bytes(val_res);

         let el = num::FromPrimitive::from_u16(val_char);

         match el {
             Some(val) => Ok(val),
             None => Err("Could not parse Licensee"),

         }

     }

 }
