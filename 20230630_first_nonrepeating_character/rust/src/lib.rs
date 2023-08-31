type AnyError = Box<dyn std::error::Error>;

fn char_to_idx(c: char) -> Result<usize, AnyError> {
    if !c.is_ascii_lowercase() {
        let msg = format!("Cannot convert {c} into an index! Must be a lowercase ASCII character");
        Err(msg.into())
    } else {
        Ok((c as usize) - 97)
    }
}

#[derive(Default, Debug, Copy, Clone)]
enum Reading {
    #[default]
    NotFound,
    FoundOnce(usize),
    FoundMultiple,
}

fn first_non_repeating_char(input: &str) -> Result<usize, AnyError> {
    let mut sensor = [Reading::default(); 26];

    for (idx, c) in input.chars().enumerate() {
        let cidx = char_to_idx(c)?;
        sensor[cidx] = match sensor[cidx] {
            Reading::NotFound => Reading::FoundOnce(idx),
            _ => Reading::FoundMultiple,
        }
    }

    sensor
        .iter()
        .filter_map(|r| match r {
            Reading::FoundOnce(idx) => Some(idx),
            _ => None,
        })
        .min()
        .copied()
        .or(Some(usize::MAX))
        .ok_or_else(|| "Something went wrong!".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_fn() {
        assert_eq!(first_non_repeating_char("abcdcaf").unwrap(),       1);
        assert_eq!(first_non_repeating_char("faadabcbbebdf").unwrap(), 6);
        assert_eq!(first_non_repeating_char("a").unwrap(),             0);
        assert_eq!(first_non_repeating_char("ab").unwrap(),            0);
        assert_eq!(first_non_repeating_char("abac").unwrap(),          1);
        assert_eq!(first_non_repeating_char("ababac").unwrap(),        5);
        assert_eq!(first_non_repeating_char("ababacc").unwrap(),       usize::MAX);
        assert_eq!(first_non_repeating_char("aabbccddeeff").unwrap(),  usize::MAX);
    }
}
