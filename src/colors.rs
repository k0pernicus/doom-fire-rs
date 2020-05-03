use lazy_static::lazy_static;
use std::collections::HashMap;

fn to_decimal(decimal: char, unit: char) -> u8 {
    HEX_TO_DECIMAL.get(&decimal).unwrap() * 16 + HEX_TO_DECIMAL.get(&unit).unwrap()
}

fn hexa_str_to_decimal(hexa: &str) -> Option<(u8, u8, u8)> {
    let c = hexa.to_lowercase().chars().collect::<Vec<char>>();
    return Some((to_decimal(c[0], c[1]), to_decimal(c[2], c[3]),to_decimal(c[4], c[5])));
}

const HEXA_COLORS: [&str; 2] = ["000000", "ffffff"];

// Convert all hexadecimal colors to triple decimal (u8) colors at compile time
lazy_static! {
    static ref HEX_TO_DECIMAL: HashMap<char, u8> = "0123456789abcdef"
        .chars()
        .enumerate()
        .map(|(counter, c)| (c, counter as u8))
        .collect();
    
    pub static ref COLORS: [(u8, u8, u8); 2] = {
        let mut a = [(0u8, 0u8, 0u8); 2];
        for i in 0..2 {
            a[i] = hexa_str_to_decimal(HEXA_COLORS[i]).unwrap();
        }
        a
    };
}