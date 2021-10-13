use std::fmt::{Display, Formatter, Result};

struct RunLengthEncoding(Vec<(char, u32)>);

impl From<&str> for RunLengthEncoding {
    fn from(s: &str) -> Self {
        let mut current_letter = match s.chars().next() {
            None => return RunLengthEncoding(Vec::new()),
            Some(l) => l,
        };
        let mut out = Vec::with_capacity(s.len() / 2);
        let mut current_count = 0;

        for c in s.chars() {
            if c == current_letter {
                current_count += 1; 
            } else {
                out.push((current_letter, current_count));
                current_count = 1;
                current_letter = c;
            }
        }
        out.push((current_letter, current_count));

        RunLengthEncoding(out)
    }
}

impl Display for RunLengthEncoding {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (letter, count) in self.0.iter() {
            write!(f, "{}{}", letter, count)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let rle: RunLengthEncoding = "AABBAACCDaaTTttcc".into();
        assert_eq!(rle.to_string(), String::from("A2B2A2C2D1a2T2t2c2"));
    }

    #[test]
    fn second_example() {
        let rle: RunLengthEncoding = "ABCAAA".into();
        assert_eq!(rle.to_string(), String::from("A1B1C1A3"));
    }

    #[test]
    fn third_example() {
        let rle: RunLengthEncoding = "TTGGCCGTCGT".into();
        assert_eq!(rle.to_string(), String::from("T2G2C2G1T1C1G1T1"));
    }

    #[test]
    fn fourth_example() {
        let rle: RunLengthEncoding = "!!..??%%..".into();
        assert_eq!(rle.to_string(), String::from("!2.2?2%2.2"));
    }
}
