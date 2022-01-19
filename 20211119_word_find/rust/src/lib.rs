use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// Given a concatenated list of words `concat` and a list of possible words to find
// `words`, identify the different combinations of words from `words` that can be
// derived from `concat` by simply adding spaces in the appropriate places.
// Each function is modified slightly to count iterations so they can be compared for
// efficiency.

/// This function operates by repeatedly stripping the a prefix from the front of
/// `concat` in a manner similar to a depth-first search.
#[allow(dead_code)]
fn find_words_strip_prefix(concat: &str, words: &[&str]) -> Vec<String> {
    let mut out = Vec::new(); // The output list
    let mut iterations = 0;

    // As long as there are still items on the stack, continue to operate. The stack
    // will contain tuples where the first item is a list of words that have been
    // successfully stripped from the front of `concat` and the second item is the
    // remaining concatenated string after words have been removed from the front.
    let mut stack = vec![(Vec::new(), concat)]; // (found_words, remaining)
    while let Some((found_words, remaining)) = stack.pop() {
        iterations += 1;
        // This serves as our 'base case'. If we ever fully 'use up' the `concat`
        // string, then we know we've made all the words we can out of it. In that
        // case, turn the list of found words into a String `word_str` and add
        // `word_str` to our output list. The alternative is that there are some
        // letters left in `remaining` that can't make a word, which means the
        // `found_words` up to this point are dropped and not added to `out`.
        if remaining.is_empty() {
            let word_str = found_words
                .iter()
                .map(|s: &&str| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            out.push(word_str);
            continue;
        }

        // For each word in our list of possible words to find...
        for word in words {
            iterations += 1;
            // If that word can be stripped from the front of `remaining`, then
            // add a new tuple to the stack by adding `word` to the list of
            // found words and stripping `word` from the front of `remaining` to
            // produce `remnant`.
            if let Some(remnant) = remaining.strip_prefix(word) {
                let mut new_found_words = found_words.clone();
                new_found_words.push(word);
                stack.push((new_found_words, remnant));
            }
        }
    }
    println!("{} iterations", iterations);
    out
}


/// This function operates similarly to `find_words_strip_prefix()`, except instead of
/// iterating repeatedly over the list of words, it iterates primarily over `concat`,
/// checking a HashSet of words for matches.
#[allow(unstable_name_collisions)]
fn find_words_char_iter(concat: &str, words: &[&str]) -> Vec<String> {
    let mut iterations = 0;
    let mut out = Vec::new();
    let words: HashSet<&str> = words.iter().copied().collect();

    let mut stack: Vec<(Vec<Vec<&str>>, usize)> = vec![(vec![Vec::new()], 0)];
    while let Some((strings, start_idx)) = stack.pop() {
        iterations += 1;
        if start_idx == concat.len() {
            out.extend(strings.clone());
            continue;
        }

        for end_idx in start_idx..=concat.len() {
            iterations += 1;
            let maybe_word = &concat[start_idx..end_idx];
            if words.contains(maybe_word) {
                let mut new_strings = strings.clone();
                new_strings.iter_mut().for_each(|s| s.push(maybe_word));
                stack.push((new_strings, end_idx));
            }
        }
    }

    println!("{} iterations", iterations);
    out.into_iter()
        .map(|s| s.into_iter().intersperse(" ").collect())
        .collect()
}


/// This version uses recursion and caching to break up `concat`. We need a recursive
/// helper function, since the original input is not sufficient.
#[allow(unstable_name_collisions)]
fn find_words_recursive(concat: &str, words: &[&str]) -> Vec<String> {
    let mut cache = HashMap::new();
    let mut iterations = 0;
    let words = words.iter().copied().collect::<HashSet<_>>();
    let result =
        find_words_recursive_helper(concat, concat.len(), &words, &mut cache, &mut iterations);
    println!("{} iterations", iterations);
    result
        .iter()
        .cloned()
        .map(|s| s.iter().copied().intersperse(" ").collect())
        .collect()
}

/// You'll notice the similarities between this function and `find_words_char_iter()`
fn find_words_recursive_helper<'a>(
    concat: &'a str,
    end_idx: usize,
    words: &HashSet<&str>,
    cache: &mut HashMap<usize, Vec<Vec<&'a str>>>,
    iterations: &mut usize,
) -> Vec<Vec<&'a str>> {
    *iterations += 1;
    let mut res = Vec::new();
    if end_idx == 0 { return vec![Vec::new()]; }
    if let Some(cached) = cache.get(&end_idx) { return cached.to_vec(); }

    for start_idx in 0..end_idx {
        *iterations += 1;
        let maybe_word = &concat[start_idx..end_idx];
        if words.contains(&maybe_word) {
            let inner_words =
                find_words_recursive_helper(concat, start_idx, words, cache, iterations);
            for mut inner_word in inner_words {
                inner_word.push(maybe_word);
                res.push(inner_word);
            }
        }
    }
    cache.insert(end_idx, res.clone());
    res
}

/// This function gathers all three of the different methods and reports the number
/// of iterations undertaken by each method.
fn find_words(concat: &str, words: &[&str]) -> Vec<String> {
    println!("Operating on concat: {}; words: {:?}", concat, words);

    print!("Using the prefix stripping method: ");
    let mut stack_result = find_words_strip_prefix(concat, words);

    print!("Using the character iteration method: ");
    let mut dp_result = find_words_char_iter(concat, words);

    print!("Using the recursive method: ");
    let mut recursive_result = find_words_recursive(concat, words);

    // Check to make sure that all three methods return the same result, otherwise
    // panic and fail any tests relying on this function.
    stack_result.sort();
    dp_result.sort();
    recursive_result.sort();
    assert_eq!(stack_result, dp_result);
    assert_eq!(dp_result, recursive_result);
    println!();

    stack_result
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_it(input: &str, word_dict: &[&str], expected: &[&str]) {
        let mut result = find_words(input, word_dict);
        result.sort_unstable();

        let mut expected = expected.to_vec();
        expected.sort_unstable();

        assert_eq!(result, expected);
    }

    #[test]
    fn should_find_single_word() {
        let input = "bears";
        let word_dict = vec!["bears"];
        let expected = vec!["bears"];
        test_it(input, &word_dict, &expected);
    }

    #[test]
    fn should_find_multiple_words() {
        let input = "bearsfish";
        let word_dict = vec!["bears", "fish"];
        let expected = vec!["bears fish"];
        test_it(input, &word_dict, &expected);
    }

    #[test]
    fn should_find_multiple_lists() {
        let input = "catsanddog";
        let word_dict = vec!["cat", "cats", "and", "sand", "dog"];
        let expected = vec!["cats and dog", "cat sand dog"];
        test_it(input, &word_dict, &expected);
    }

    #[test]
    fn should_find_multiple_lists_with_reuse() {
        let input = "pineapplepenapple";
        let word_dict = vec!["apple", "pen", "applepen", "pine", "pineapple"];
        let expected = vec![
            "pine apple pen apple",
            "pineapple pen apple",
            "pine applepen apple",
        ];
        test_it(input, &word_dict, &expected);
    }

    #[test]
    fn should_return_empty_vec_for_no_matches() {
        let input = "catsandog";
        let word_dict = vec!["cats", "dog", "sand", "and", "cat"];
        let expected: Vec<&str> = vec![];
        test_it(input, &word_dict, &expected);
    }

    #[test]
    fn should_find_compound_words() {
        let input = "baseball";
        let word_dict = vec!["base", "ball", "baseball"];
        let expected = vec!["baseball", "base ball"];
        test_it(input, &word_dict, &expected);
    }

    #[test]
    fn benchmarking() {
        let input = "aaaaaaaaaa";
        let word_dict = vec![
            "a",
            "aa",
            "aaa",
            "aaaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaa",
            "aaaaaaaaaa",
        ];
        let _ = find_words(input, &word_dict);
    }
}
