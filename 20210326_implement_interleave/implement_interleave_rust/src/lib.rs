//! # Friday Coding Challenge
//! 
//! Write the code in any language you like. Don't worry if someone has 
//! completed the challenge. Write and post your solution. Daily practice makes 
//! progress (and good habits).
//! 
//! ## Combine Alternating Lists
//! 
//! Write a method that combines two lists by alternatingly taking elements.
//! For example: given the two lists [a, b, c] and [1, 2, 3], 
//! the method should return [a, 1, b, 2, c, 3]

use std::fmt;
use std::iter::Map;

/// Enum type capable of holding two different types of variables
pub enum Union<A, B> {
    A(A),
    B(B),
}

/// Implementation of debug printing that makes the printing of these values
/// prettier.
impl<A, B> fmt::Debug for Union<A, B> 
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Union::A(a) => write!(f, "{}", a),
            Union::B(b) => write!(f, "{}", b),
        }
    }
}

/// Given two slices of two potentially different types of items, returns a Vec
/// containing a Union of the two types, interleaved such that a[1, 2, 3] and 
/// b[9, 8, 7] yields a|b[1, 9, 2, 8, 3, 7]. In cases where a or b is longer, the
/// excess items are appended to the end of the resulting Vec.
pub fn interleave<A, B>(a: &[A], b: &[B]) -> Vec<Union<A, B>>
where
    A: Copy,
    B: Copy,
{
    let a_is_longer = a.len() > b.len();
    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut out: Vec<Union<A, B>> = Vec::new();

    loop {
        // If we haven't exhausted `a`, add its next element to `out`. If we have,
        // and `b` is longer, append the rest of `b` to `out` and return.
        match a.get(a_idx) {
            Some(x) => {
                out.push(Union::A(*x));
                a_idx += 1;
            }
            None => {
                if !a_is_longer {
                    let remaining_b: Map<_, _> = b[b_idx..].iter().map(|x| Union::B(*x));
                    out.extend(remaining_b);
                    return out;
                }
            }
        }


        // If we haven't exhausted `b`, add its next element to `out`. If we have,
        // and `a` is longer, append the rest of `a` to `out` and return.
        match b.get(b_idx) {
            Some(x) => {
                out.push(Union::B(*x));
                b_idx += 1;
            }
            None => {
                if a_is_longer {
                    let remaining_a: Map<_, _> = a[a_idx..].iter().map(|x| Union::A(*x));
                    out.extend(remaining_a);
                    return out;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_lengths() {
        let a = vec!['a', 'b', 'c'];
        let b = vec![1, 2, 3];
        assert_eq!(format!("{:?}", interleave(&a, &b)), r#"[a, 1, b, 2, c, 3]"#)
    }

    #[test]
    fn test_longer_a() {
        let a = vec!['a', 'b', 'c', 'd', 'e'];
        let b = vec![1, 2, 3];
        assert_eq!(format!("{:?}", interleave(&a, &b)), r#"[a, 1, b, 2, c, 3, d, e]"#)
    }

    #[test]
    fn test_longer_b() {
        let a = vec!['a', 'b', 'c'];
        let b = vec![1, 2, 3, 4, 5];
        assert_eq!(format!("{:?}", interleave(&a, &b)), r#"[a, 1, b, 2, c, 3, 4, 5]"#)
    }
}
