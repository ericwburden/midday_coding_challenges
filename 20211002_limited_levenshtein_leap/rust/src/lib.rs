use std::collections::VecDeque;

/// Function to test two strings, indicate whether they are one or fewer letters appart
fn close_enough(word1: &str, word2: &str) -> bool {
    let differing_letters: u8 = word1.chars()
        .zip(word2.chars())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum();
    differing_letters <= 1
}

fn find_transform_sequence<'a>(start: &'a str, end: &str, dictionary: &[&'a str]) -> Option<Vec<&'a str>> {
    // We're doing a Breadth-First Search for this one. So, we start with a vector
    // indicating which entries have been visited, and a VecDeque that we can use as
    // a FIFO data structure. Inside the VecDeque, we're keeping Tuples that contain
    // the word and the path followed to get to the word
    let mut visited = vec![false; dictionary.len()];
    let mut deque = VecDeque::new();
    deque.push_back((start, vec![start]));

    // So long as there is something in the VecDeque, we haven't checked all the 
    // possible 'nodes', so keep going...
    while let Some((word, path)) = deque.pop_front() {
        // If the word we popped off the VecDeque is our target word, return the path
        // to it
        if word == end { return Some(path); }
        
        // Otherwise, check each word in the dictionary against the current word from
        // the VecDeque. If it hasn't been visited yet, mark it as visited, and add the
        // word and the path to the word to the VecDeque
        for (idx, dict_word) in dictionary.iter().enumerate() {
            if visited[idx] { continue; }
            if close_enough(word, dict_word) {
                visited[idx] = true;
                let mut path_to_here = path.clone();
                path_to_here.push(dict_word);
                deque.push_back((dict_word, path_to_here));
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_case() {
        let start = "car";
        let end = "cat";
        let words = ["cat"];
        let result = find_transform_sequence(start, end, &words);
        let expected = Some(vec!["car", "cat"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_case() {
        let start = "dog";
        let end = "cat";
        let words = ["dop", "dot", "cop", "cap", "car", "cat"];
        let result = find_transform_sequence(start, end, &words);
        let expected = Some(vec!["dog", "dop", "cop", "cap", "cat"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn handle_no_solution() {
        let start = "bears";
        let end = "share";
        let words = ["truck", "frond", "crank", "drops", "blame", "share"];
        let result = find_transform_sequence(start, end, &words);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn find_shortest_of_two_paths() {
        let start = "food";
        let end = "dung";
        let words = ["frod", "frog", "grog", "guog", "gung", "dung", "fond", "fund", "fung"];
        let result = find_transform_sequence(start, end, &words);
        let expected = Some(vec!["food", "fond", "fund", "fung", "dung"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn find_path_when_a_cycle_exists() {
        let start = "tot";
        let end = "kid";
        let words = ["tod", "tid", "tim", "tom", "kim", "kid"];
        let result = find_transform_sequence(start, end, &words);
        let expected = Some(vec!["tot", "tod", "tid", "kid"]);
        assert_eq!(result, expected);
    }
}
