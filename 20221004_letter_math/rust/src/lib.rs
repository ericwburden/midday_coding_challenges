use itertools::*;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

/// This trait tests a collection to determine if all values
/// in the collection are unique. Returns `false` if any values
/// are duplicated.
pub trait AllUnique {
    fn all_unique(&self) -> bool;
}

impl<T: Eq + Hash> AllUnique for Vec<T> {
    fn all_unique(&self) -> bool {
        let mut item_set = HashSet::new();
        for item in self.iter() {
            if item_set.contains(item) { return false; }
            item_set.insert(item);
        }
        true
    }
}

/// Provides a mapping from letter to value
#[derive(Debug)]
struct Replacements(HashMap<char, u32>);

impl Replacements {
    /// Provides access to the inner `HashMap`'s .get() function.
    pub fn get(&self, k: &char) -> Option<&u32> {
        self.0.get(k)
    }
}

/// Conversion from a list of <letter, value> pairs into a `Replacements`
/// mapping.
impl From<Vec<(char, u32)>> for Replacements {
    fn from(input: Vec<(char, u32)>) -> Self {
        let mut out = HashMap::new();
        for (letter, value) in input {
            out.insert(letter, value);
        }
        Replacements(out)
    }
}

/// Represents a 'word' in the expression being evaluated. Allows for
/// replacing each letter with a decimal digit and evaluating the result
/// to a single number.
struct Word(Vec<char>);

impl From<&str> for Word {
    fn from(input: &str) -> Self {
        Word(input.chars().collect())
    }
}

impl Word {
    /// Replace letters in `Word` with the numbers specified in `replacements`
    /// and, if successful, return a result wrapping the number produced.
    pub fn try_replace(&self, replacements: &Replacements) -> Result<u32, &'static str> {
        let mut total = 0;

        for (idx, c) in self.0.iter().rev().enumerate() {
            let pow = 10u32.pow(idx as u32);
            let value = replacements.get(c).ok_or("Replacement not found")?;
            total += pow * value
        }

        Ok(total)
    }

    /// Provide access to the inner `Vec`'s .iter() function.
    pub fn iter(&self) -> std::slice::Iter<char> {
        self.0.iter()
    }
}

/// Represents an expression adding two words and yielding a 
/// third word. Currently assumes addition.
struct Expression {
    input1: Word,
    input2: Word,
    result: Word,
    letters: Vec<char>,
    options: Vec<Vec<u32>>,
}

impl Expression {
    fn new(input1: Word, input2: Word, result: Word) -> Self {
        let letters = input1
            .iter()
            .chain(input2.iter())
            .chain(result.iter())
            .unique()
            .copied()
            .collect::<Vec<_>>();

        let options = vec![(0..=9).collect(); letters.len()];

        Expression { input1, input2, result, letters, options }
    }

    /// Attempt to solve the `Expression` by trying all combinations of letter 
    /// replacements in each `Word` and evaluating the result.
    fn try_solve(&self) -> Result<Replacements, &'static str> {
        // Start by getting all the unique letters
        let possibilities = self.options
            .iter()
            .multi_cartesian_product()
            .filter(|v| v.all_unique())
            .map(|v| v.into_iter().copied());

        for possibility in possibilities {
            let pairs: Vec<(char, u32)> = self.letters.iter().cloned().zip(possibility).collect();
            let replacements = Replacements::from(pairs);

            let input1_replaced = self.input1.try_replace(&replacements)?;
            let input2_replaced = self.input2.try_replace(&replacements)?;
            let result_replaced = self.result.try_replace(&replacements)?;
            if input1_replaced + input2_replaced == result_replaced {
                return Ok(replacements);
            }
        }

        Err("Could not solve expression!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        // elf + elf = fool
        let expression = Expression::new(
            Word::from("elf"),
            Word::from("elf"),
            Word::from("fool"),
        );

        let solution = expression.try_solve().expect("There is a solution");
        assert_eq!(*solution.get(&'e').expect("e has a replacement value"), 7);
        assert_eq!(*solution.get(&'l').expect("l has a replacement value"), 2);
        assert_eq!(*solution.get(&'f').expect("f has a replacement value"), 1);
        assert_eq!(*solution.get(&'o').expect("o has a replacement value"), 4);
    }

    #[test]
    fn second_example() {
        // egg + egg = page
        let expression = Expression::new(
            Word::from("egg"),
            Word::from("egg"),
            Word::from("page"),
        );

        let solution = expression.try_solve().expect("There is a solution");
        assert_eq!(*solution.get(&'e').expect("e has a replacement value"), 8);
        assert_eq!(*solution.get(&'g').expect("g has a replacement value"), 9);
        assert_eq!(*solution.get(&'p').expect("p has a replacement value"), 1);
        assert_eq!(*solution.get(&'a').expect("a has a replacement value"), 7);
    }

    #[test]
    fn third_example() {
        // ears + ears = swear
        let expression = Expression::new(
            Word::from("ears"),
            Word::from("ears"),
            Word::from("swear"),
        );

        let solution = expression.try_solve().expect("There is a solution");
        assert_eq!(*solution.get(&'e').expect("e has a replacement value"), 8);
        assert_eq!(*solution.get(&'a').expect("a has a replacement value"), 4);
        assert_eq!(*solution.get(&'r').expect("r has a replacement value"), 2);
        assert_eq!(*solution.get(&'s').expect("s has a replacement value"), 1);
        assert_eq!(*solution.get(&'w').expect("w has a replacement value"), 6);
    }

    #[test]
    fn fourth_example() {
        // top + tot = opt
        let expression = Expression::new(
            Word::from("top"),
            Word::from("tot"),
            Word::from("opt"),
        );

        let solution = expression.try_solve().expect("There is a solution");
        assert_eq!(*solution.get(&'t').expect("t has a replacement value"), 2);
        assert_eq!(*solution.get(&'o').expect("o has a replacement value"), 5);
        assert_eq!(*solution.get(&'p').expect("p has a replacement value"), 0);
    }

    #[test]
    fn fifth_example() {
        // she + eel = else
        let expression = Expression::new(
            Word::from("she"),
            Word::from("eel"),
            Word::from("else"),
        );

        let solution = expression.try_solve().expect("There is a solution");
        assert_eq!(*solution.get(&'s').expect("s has a replacement value"), 9);
        assert_eq!(*solution.get(&'h').expect("h has a replacement value"), 8);
        assert_eq!(*solution.get(&'e').expect("e has a replacement value"), 1);
        assert_eq!(*solution.get(&'l').expect("l has a replacement value"), 0);
    }
}
