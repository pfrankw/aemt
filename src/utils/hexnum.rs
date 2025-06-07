
pub fn parse_hex_u16(s: &str) -> Result<u16, std::num::ParseIntError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u16::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u16>()
    }
}

pub fn parse_hex_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u32>()
    }
}

