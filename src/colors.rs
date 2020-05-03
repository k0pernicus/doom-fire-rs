use lazy_static::lazy_static;
use std::collections::HashMap;

fn to_decimal(decimal: char, unit: char) -> u8 {
    HEX_TO_DECIMAL.get(&decimal).unwrap() * 16 + HEX_TO_DECIMAL.get(&unit).unwrap()
}

fn hexa_str_to_decimal(hexa: &str) -> Option<(u8, u8, u8)> {
    let c = hexa.to_lowercase().chars().collect::<Vec<char>>();
    return Some((
        to_decimal(c[0], c[1]),
        to_decimal(c[2], c[3]),
        to_decimal(c[4], c[5]),
    ));
}

const HEXA_COLORS: [&str; 35] = [
    "070707", "220606", "330D05", "4E0802", "5F1000", "701700", "811400", "9B1900", "AD2100",
    "BD3400", "CE3C00", "D73A00", "F14000", "F14A00", "E75600", "E75F00", "DD6900", "DC7200",
    "DB7B00", "DA8300", "D18400", "D08D00", "CF9500", "C49E00", "C49E00", "C3A600", "C3A600",
    "C2AF00", "B8AF00", "B6B800", "B6B800", "CED05E", "DEDF97", "EFEFC2", "FFFFFF",
];

// Convert all hexadecimal colors to triple decimal (u8) colors at compile time
lazy_static! {
    static ref HEX_TO_DECIMAL: HashMap<char, u8> = "0123456789abcdef"
        .chars()
        .enumerate()
        .map(|(counter, c)| (c, counter as u8))
        .collect();
    pub static ref COLORS: [(u8, u8, u8); 35] = {
        let mut a = [(0u8, 0u8, 0u8); 35];
        for i in 0..35 {
            a[i] = hexa_str_to_decimal(HEXA_COLORS[i]).unwrap();
        }
        a
    };
}
