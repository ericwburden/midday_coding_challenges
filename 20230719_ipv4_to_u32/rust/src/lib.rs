use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct IPv4([u8; 4]);

impl From<IPv4> for u32 {
    fn from(value: IPv4) -> Self {
        u32::from_ne_bytes(value.0)
    }
}

impl From<u32> for IPv4 {
    fn from(value: u32) -> Self {
        let parts = value.to_ne_bytes();
        IPv4(parts)
    }
}

impl FromStr for IPv4 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = [0; 4];
        for (idx, str_part) in s.split('.').enumerate() {
            if idx > 3 {
                return Err("Can only convert a string in the format of ###.###.###.###");
            }
            parts[3 - idx] = str_part
                .parse::<u8>()
                .map_err(|_| "Could not parts a set of numbers!")?;
        }
        Ok(IPv4(parts))
    }
}

impl fmt::Display for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [p1, p2, p3, p4] = self.0;
        write!(f, "{p4}.{p3}.{p2}.{p1}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = 2149583361;
        let test_str = "128.32.10.1";
        let test_ipv4 = IPv4::from(test_case);
        assert_eq!(test_ipv4.to_string(), test_str);

        let test_back = u32::from(test_ipv4);
        assert_eq!(test_case, test_back);

        let from_str = IPv4::from_str(test_str).unwrap();
        assert_eq!(test_ipv4, from_str);
    }
}
