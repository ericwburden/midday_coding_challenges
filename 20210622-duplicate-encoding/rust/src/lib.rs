use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// Creates a HashMap from a word, marking duplicates
///
/// Duplicate letters will have a value `true`, non-duplicates will be `false`
pub fn build_letter_map(word: &str) -> HashMap<char, bool> {
    let mut out = HashMap::with_capacity(word.len());
    word.chars().for_each(|c| {
        match out.entry(c) {
            Entry::Occupied(mut o) =>  { o.insert(true); },
            Entry::Vacant(v) => { v.insert(false); },
        }
    });
    out
}


/// Encode a character based on `map`
pub fn encode_char(c: &char, map: &HashMap<char, bool>) -> char {
    if let Some(true) = map.get(c) { ')' } else { '(' }
}


/// Duplicate encodes a string
///
/// The goal of this exercise is to convert a string to a new string where each
/// character in the new string is "(" if that character appears only once in the
/// original string, or ")" if that character appears more than once in the original
/// string. Ignore capitalization when determining if a character is a duplicate.
///
/// ```
/// # use rust::duplicate_encode;
/// assert_eq!(duplicate_encode("Sweet Beets"), ")()))(())))")
/// ```
pub fn duplicate_encode(word: &str) -> String {
    let lower_word = word.to_lowercase();
    let letter_map = build_letter_map(&lower_word);
    lower_word.chars().map(|c| encode_char(&c, &letter_map)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_encodes_strings() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())");
        assert_eq!(duplicate_encode("(( @"), "))((");
        assert_eq!(duplicate_encode("\n\n"), "))");
    }
}
