use std::cmp::Ordering;

/// This approach will 'tokenize' the input string, converting it to a data structure
/// whose nesting depth can be more easily calculated. This happens by walking down
/// the input string and converting characters into data structures called `Token`s.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    OpenParentheses,
    CloseParentheses,
    String(String),
    SingleNested(Box<Token>),
    MultiNested(Vec<Token>)
}

impl Token {
    /// Each token can report it's own nesting depth. For anything but a `Nested`
    /// token, the depth is 0. Otherwise, it's 1 plus the depth of the token
    /// nested within it.
    fn depth(&self) -> usize {
        match self {
            Token::SingleNested(t) => 1 + t.depth(),
            Token::MultiNested(t) => 1 + t.iter().map(|x| x.depth()).max().unwrap_or(0),
            _ => 0
        }
    }
}

/// Rust *Traits* are kind of like inheritance, except you can just attach new functions
/// to existing structs with them. Here, I'm attaching the same `wrap()` function to 
/// both `String`s and `Token`s, so I can call `.wrap()` on either without knowing
/// which one it is. It's basically an interface.
trait Wrap {
    fn wrap(self) -> Token;
}

impl Wrap for String {
    fn wrap(self) -> Token { Token::String(self) }
}

impl Wrap for Token {
    fn wrap(self) -> Token { Token::SingleNested(Box::new(self)) }
}

impl Wrap for Vec<Token> {
    fn wrap(self) -> Token { Token::MultiNested(self) }
}

/// This trait allows for splitting 'around' a element in a vector. The original
/// vector is truncated prior to the given index, and the returned vector includes
/// all the elements after the given index until the end.
trait SplitAround<T> {
    fn split_around(&mut self, idx: usize) -> Vec<T>;
}

impl<T> SplitAround<T> for Vec<T> {
    fn split_around(&mut self, idx: usize) -> Vec<T> {
        match self.len().cmp(&idx) {
            // This should probably be an error instead of a panic, but I'm tired of
            // tweaking and it passes all the tests as-is. :D
            Ordering::Less => panic!("Cannot split around indices greater than vector length!"),

            Ordering::Equal => {
                _ = self.pop();
                return Vec::new();
            },

            Ordering::Greater => {
                let result = self.split_off(idx + 1);
                _ = self.pop();
                return result;
            }
        }
    }
}

/// The main function here, tokenizes (and kind of parses) the input string into a 
/// `Vec<Token>`, where each `Token` knows it's own depth. This will make getting the
/// max depth _much_ easier. 
fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // Keeps track of the indices of open parentheses in the list of `Token`s
    let mut open_parens = Vec::new();

    // Because we may have one or more characters nested inside parentheses, we'll use
    // this `string_buffer` to group them together so they can be treated as a single
    // unit instead of a collection of characters.
    let mut string_buffer = String::new();

    // For each character in the input string...
    for c in s.chars() {
        // First off, if the character is a parentheses, then we know that any string
        // of characters we were seeing is now done, so we go ahead and add the contents
        // of `string_buffer` to our list of `Token`s (wrapped in a Token, of course)
        // and clear the `string_buffer` so we can use it again later for more characters.
        if (c == '(' || c == ')') && !string_buffer.is_empty() {
            tokens.push(string_buffer.wrap());
            string_buffer = String::new();
        }

        // Now, we do different things depending on exactly which character we find. 
        // Clearly, ')' is the most complicated.
        match c {
            // Just add a token representing an open parentheses to our list of 
            // `Token`s. We need to do this because Rust won't let us put two different
            // types in the same `Vec`. Add to `open_parens` the index in `tokens` where
            // this open parentheses was added. This lets us keep track of whether and 
            // where we have 'lurking' open parentheses.
            '(' => {
                tokens.push(Token::OpenParentheses);
                open_parens.push(tokens.len() - 1);
            },

            // Now comes the tricky bit. What happens when a closing parentheses is 
            // found depends on whether there are any opening parentheses lurking back
            // in the stack...
            ')' => match open_parens.pop() {
                // If there are no opening parentheses in the stack, just add the 
                // closing parentheses to the stack and move forward.
                None => tokens.push(Token::CloseParentheses),

                // If there are any opening parentheses that can be closed to form a
                // nested token, then we take everything between that opening 
                // parentheses and the end of the stack, wrap it in a Token, and add
                // the new nested token back to the stack.
                Some(oidx) => {
                    let mut to_nest = tokens.split_around(oidx);

                    // Adds a SingleNested token if there's only one token between the
                    // opening and closing parentheses, otherwise adds a MultiNested
                    // token.
                    let wrapped = match to_nest.len() {
                        0 => continue,
                        1 => to_nest.drain(0..).next().unwrap().wrap(),
                        _ => to_nest.wrap(),
                    };
                    tokens.push(wrapped);
                }
            },

            // Finally, if the character is anything besides a parentheses, just add it
            // to the string buffer.
            _ => string_buffer.push(c),
        }
    }

    // It's possible to have dangling characters at the end of the string. If so, we
    // just push the contents of the string buffer to the end of our list of tokens.
    // This won't actually affect our nesting depth, but it felt wrong to leave them off.
    if !string_buffer.is_empty() { tokens.push(string_buffer.wrap()) }

    tokens // Return the list of tokens
}

/// With the tokenizer in place, now we just need to parse our input string into a list
/// of tokens, get the depth of each token in the list, and return the maximum (or 0
/// if there are no tokens in the list).
#[allow(unused)]
fn nesting_depth(s: &str) -> usize {
    tokenize(s)
        .into_iter()
        .map(|x| x.depth())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_nesting_depth(s: &str, e: usize) {
        let result = nesting_depth(s);
        assert_eq!(result, e);
    }

    #[test]
    /// Empty strings return a nesting depth of zero
    fn test_case_one() { test_nesting_depth("", 0); }

    #[test]
    /// Empty parentheses do not contribute to nesting depth
    fn test_case_two() { test_nesting_depth("()", 0); }

    #[test]
    /// Nesting depth should be the number of pairs of enclosing parentheses
    fn test_case_three() { test_nesting_depth("(I)", 1); }

    #[test]
    /// Parentheses not in a pair do not contribute to nesting depth
    fn test_case_four() { 
        test_nesting_depth("(((A(((", 0); 
        test_nesting_depth(")))A)))", 0); 
    }

    #[test]
    /// Extra opening parentheses can be ignored
    fn test_case_five() { 
        test_nesting_depth("(((A)", 1);
        test_nesting_depth("(A)((", 1);
        test_nesting_depth("((A))((B)", 2);
    }

    #[test]
    /// Extra closing parentheses can be ignored
    fn test_case_six() { test_nesting_depth("(((A))))))))", 3); }

    #[test]
    /// There may be multiple string parts nested at different levels
    fn test_case_seven() { 
        test_nesting_depth("((hey))(bro)", 2); 
        test_nesting_depth("(hey)((sis))", 2); 
    }

    #[test]
    /// Parentheses only make a pair with the closest opposite parentheses
    fn test_case_eight() { test_nesting_depth("()(a)()", 1); }

    #[test]
    /// Parentheses only make a pair with the closest opposite parentheses
    fn test_case_nine() { test_nesting_depth("(()a())", 1); }

    #[test]
    /// Parentheses only make a pair with the closest opposite parentheses
    fn test_case_ten() { test_nesting_depth("(()(a)())", 2); }

    #[test]
    fn test_super_nested() { test_nesting_depth("a(b(c(d(e(f(g(h(i)j)k)l)m)n)o)p)q", 8) }
}
