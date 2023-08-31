const RAIN_FACTORS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn convert(number: u32) -> String {
    let mut sounds = String::with_capacity(15);
    for (factor, sound) in RAIN_FACTORS {
        if number % factor > 0 {
            continue;
        }
        sounds.push_str(sound);
    }

    if sounds.is_empty() {
        return number.to_string();
    }
    sounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const TEST_CASES: [(u32, &str); 7] = [
        (1,   "1"),
        (3,   "Pling"),
        (5,   "Plang"),
        (15,  "PlingPlang"),
        (21,  "PlingPlong"),
        (105, "PlingPlangPlong"),
        (113, "113"),
    ];

    #[test]
    fn run_tests() {
        for (input, output) in TEST_CASES {
            assert_eq!(convert(input), output);
        }
    }
}
