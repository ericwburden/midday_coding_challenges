struct Mapping {
    pub decimal: u32,
    pub roman: &'static str
}

impl Mapping {
    pub const fn new(decimal: u32, roman: &'static str) -> Self{
        Mapping { decimal, roman }
    }
}

type MappingArray = [Mapping; 14];

const DECIMAL_TO_ROMAN_MAP: MappingArray = [
    Mapping::new(1000, "M"),
    Mapping::new(900,  "CM"),
    Mapping::new(500,  "D"),
    Mapping::new(400,  "CD"),
    Mapping::new(100,  "C"),
    Mapping::new(90,   "XC"),
    Mapping::new(50,   "L"),
    Mapping::new(40,   "XL"),
    Mapping::new(10,   "X"),
    Mapping::new(9,    "IX"),
    Mapping::new(5,    "V"),
    Mapping::new(4,    "IV"),
    Mapping::new(1,    "I"),
    Mapping::new(0,    "nulla")
];

#[derive(Debug, PartialEq, Eq)]
struct RomanNumeral(std::string::String);

impl RomanNumeral {
    #[allow(dead_code)]
    pub fn new(input: &str) -> Self {
        let s = input.to_string();
        RomanNumeral(s)
    }
}

impl From<u32> for RomanNumeral {
    fn from(n: u32) -> RomanNumeral {
        let mut remainder = n;
        let mut map_idx = 0;
        let mut output = String::new();

        while remainder > 0 && map_idx < DECIMAL_TO_ROMAN_MAP.len() {
            let mapping = &DECIMAL_TO_ROMAN_MAP[map_idx];
            if remainder >= mapping.decimal {
                remainder -= mapping.decimal;
                output.push_str(mapping.roman);
            } else {
                map_idx += 1;
            }
        }
        RomanNumeral(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_conversion {
        ( $left:expr, $right:expr ) => {{
            RomanNumeral::from($left) == RomanNumeral::new($right)
        }};
    }

    #[test]
    fn test_i() {
        assert_conversion!(1, "I");
    }

    #[test]
    fn test_nulla() {
        assert_conversion!(0, "nulla");
    }

    #[test]
    fn test_xxxix() {
        assert_conversion!(39, "XXXIX");
    }

    #[test]
    fn test_cdxliv() {
        assert_conversion!(444, "CDXLIV");
    }

    #[test]
    fn test_mmmcmxcix() {
        assert_conversion!(3999, "MMMCMXCIX");
    }
}
