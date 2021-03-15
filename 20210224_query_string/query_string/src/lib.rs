//! Implement a function isMember. It takes an input list of strings called words eg. ["foo", 
//! "bar", "baz"] and an input string called query eg. "foo", It should return true if a query 
//! matches any string in words. If a query includes * it's considered a wildcard. ie. it matches 
//! exactly one character of any value at the current index of the string. You may assume that the 
//! input will not contain empty strings or empty lists.
//! 
//! # Examples
//! 
//! ```ignore
//! Input: words = ["bar", "foo", "baz"], query = "foo"
//! Output: true
//! =============================
//! Input: words = ["foo", "bar", "baz"], query = "cat" 
//! Output: false
//! =============================
//! Input: words = ["bar", "foo", "baz"], query = "**z" ,
//! Output: true
//! ```

/// Given a slice of string slices `words` and a string slice `query`, determine whether there are
/// any strings in `words` that match the given `query` string. The `query` string supports a 
/// wildcard character ('*') that can stand in for any other character.
#[rustfmt::skip]
pub fn is_member(words: &[&str], query: &str) -> bool {
    let query_chars: Vec<_> = query.chars().collect(); // `query` string to character vec

    // For each `word` in `words`...
    'word: for word in words {

        // For each (index, character) in `word`...
        for (idx, word_char) in word.chars().enumerate() {
            if query_chars[idx] == '*' { continue; }       // Wildcards match everything
            if word_char == query_chars[idx] { continue; } // Character match
            
            // If it's not a wildcard or character match, move on to the next `word`
            continue 'word;
        }

        // If we made it all the way through a word matching character-for-character,
        // then we've found a match and can return early. We only get to this point
        // if the inner loop doesn't hit `continue 'word;`
        return true;
    }
    false // Checked all the words, no matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_member(&["bar", "foo", "baz"], "foo"), true);
        assert_eq!(is_member(&["bar", "foo", "baz"], "cat"), false);
        assert_eq!(is_member(&["bar", "foo", "baz"], "**z"), true);
    }
}
