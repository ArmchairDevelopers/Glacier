use std::fmt::{self, Debug, Formatter};

// UUID v3
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.replace("-", "");
        let data1 = u32::from_str_radix(&s[0..8], 16).unwrap();
        let data2 = u16::from_str_radix(&s[8..12], 16).unwrap();
        let data3 = u16::from_str_radix(&s[12..16], 16).unwrap();
        let data4 = [
            u8::from_str_radix(&s[16..18], 16).unwrap(),
            u8::from_str_radix(&s[18..20], 16).unwrap(),
            u8::from_str_radix(&s[20..22], 16).unwrap(),
            u8::from_str_radix(&s[22..24], 16).unwrap(),
            u8::from_str_radix(&s[24..26], 16).unwrap(),
            u8::from_str_radix(&s[26..28], 16).unwrap(),
            u8::from_str_radix(&s[28..30], 16).unwrap(),
            u8::from_str_radix(&s[30..32], 16).unwrap(),
        ];

        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

impl Debug for Guid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
