// Given a string of round, curly, and square open and closing brackets, return whether
// the brackets are balanced (well-formed).
// 
// For example, given the string "([])[]({})", you should return true.
// 
// Given the string "([)]" or "((()", you should return false.
use phf::{Map, phf_map};

static BRACKETS: Map<char, char> = phf_map! {
    ')' => '(',
    ']' => '[',
    '}' => '{',
};

pub fn brackets_are_balanced(brackets: &str) -> bool {
    let mut stack = Vec::with_capacity(brackets.len());
    for c in brackets.chars() {
        match BRACKETS.get(&c) {
            Some(b) => if b == stack.last().unwrap_or(&'0') {
                stack.resize(stack.len() - 1, '\0');
            } else {
                stack.push(c);
            },
            None => if ['(', '[', '{'].contains(&c) { stack.push(c) }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
       assert_eq!(brackets_are_balanced("{([[](()){}])()}{[]()}"), true);
    }

    #[test]
    fn test_unbalanced() {
        assert_eq!(brackets_are_balanced("{{()[[]()]}(}"), false);
    }

    #[test]
    fn test_other_chars() {
        assert_eq!(brackets_are_balanced("(5 + 2) * (6 + 4)"), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(brackets_are_balanced(")))((("), false);
    }
}

