//! # It's a Number!:
//! 
//! Well, it's that time of year again: Time to review employee time-cards! Unfortunately, you 
//! work for a company that employs a large number of malicious software quality testers, and they 
//! like to enter their time "creatively". Your job is to check the time entries and figure out 
//! which ones are valid entries and which ones are not, so management can bilk appropriately 
//! refrain from compensating these mischievous employees for invalid time entries.
//! 
//! Given a string, indicate whether it represents a number. The string may be of any length, and 
//! may contain non-numeric characters such as periods, commas, preceding dashes (minus signs), 
//! and scientific notation characters, i.e. any reasonable numeric format (does not include Roman 
//! numerals). Importantly, numbers don't have spaces. You don't need to determine what *type* of 
//! number (integer, float) or the numeric value, but you do get bonus points for not using a 
//! regular expression! Extra street cred if you don't use your language's built-in number 
//! identification function (if it has one).
//! 
//! ## Examples:
//! 
//! ```ignore
//! is_numeric("10") -> true        // positive integer
//! is_numeric("-10") -> true       // negative integer
//! is_numeric("10.1") -> true      // positive decimal number
//! is_numeric("1,000,000") -> true // a big number
//! is_numeric("1e5") -> true       // a valid scientific notation
//! is_numeric("5x10-3") -> true    // also valid scientific notation
//! is_numeric("a") -> false        // that's a letter
//! is_numeric("x1") -> false       // 'x' or 'e' should be preceded by a number to be valid
//! is_numeric("a -2") -> false     // just no
//! is_numeric("-") -> false        // no numbers at all
//! is_numeric("5 00") -> false     // numbers don't have spaces
//! ```

#[rustfmt::skip]
pub fn is_numeric(input: &str) -> bool {
    let mut numerals_found = 0;
    let num_chars: Vec<_> = ('0'..='9').collect();
    for (idx, c) in input.chars().enumerate() {
        match c {
            '0'..='9' => { numerals_found += 1; continue; }
            '-' => {
                if idx == 0 { continue; }
                if &input[(idx-1)..idx] == "e" { continue; }
                if &input[(idx-3)..idx] == "x10" { continue; }
                return false;
            }
            '.' => { continue; }  // Just skip these, will need a numeral to be valid
            ',' => { // If the three chars before and after are numbers, it's ok.
                // I decided a comma needs three numbers after
                if idx + 4 > input.len() { return false; }  
                let start_idx = if idx >= 3 { idx - 3 } else { 0 };
                let preceding_three = input[start_idx..idx]
                    .chars()
                    .all(|x| num_chars.contains(&x));
                let next_three = input[(idx+1)..(idx+4)]
                    .chars()
                    .all(|x| num_chars.contains(&x));
                if !preceding_three || !next_three { return false; }
                continue;
            }
            'e' | 'E' => {
                // Can't be the first or last char
                if idx >= input.len() || idx == 0 { return false; } 

                // next char must be a number
                if !num_chars.contains(&input.chars().nth(idx+1).unwrap()) { return false; } 

                // prev char must be a number
                if !num_chars.contains(&input.chars().nth(idx-1).unwrap()) { return false; } 
                continue;
            }
            'x' => {
                // Can't be the first or next to last char
                if idx == 0 || idx > input.len() - 1 { return false; } 

                 // prev char must be a number
                if !num_chars.contains(&input.chars().nth(idx-1).unwrap()) { return false; }

                // 'x' should be followed by '10'
                if &input[(idx+1)..(idx+3)] != "10" { return false; } 
                continue;
            }
            _ => { return false; }
        };
    }

    numerals_found > 0
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn check_my_work() {
        assert_eq!(is_numeric("10"), true);
        assert_eq!(is_numeric("-10"), true);
        assert_eq!(is_numeric("10.1"), true);
        assert_eq!(is_numeric("1,000,000"), true );
        assert_eq!(is_numeric("1e5"), true);
        assert_eq!(is_numeric("5x10-3"), true);
        assert_eq!(is_numeric("a"), false );
        assert_eq!(is_numeric("x1"), false);
        assert_eq!(is_numeric("a -2"), false);
        assert_eq!(is_numeric("-"), false);
        assert_eq!(is_numeric("5 00"), false);
    }
}
