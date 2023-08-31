#![feature(allow_internal_unstable)]
use std::iter::Extend;

/// Using an enum with embedded values allows me to treat single values and
/// arbitrarily nested values as the same, at least as far as the type system
/// is concerned. I can use 'match' statements where necessary to differentiate
/// between the two.
#[derive(Debug, Clone)]
pub enum Nested<T: Copy> {
    Vec(NestedVec<T>),
    Value(T),
}

pub type NestedVec<T> = Vec<Nested<T>>;

/// Trait providing the `.flatten()` function for nested elements
pub trait Flatten<T>
where
    T: Copy,
{
    fn flatten(&self) -> Vec<T>;
}

impl<T: Copy> Flatten<T> for Nested<T> {
    fn flatten(&self) -> Vec<T> {
        let mut output = Vec::new();
        match self {
            Nested::Value(val) => output.push(*val),
            Nested::Vec(vec) => vec
                .iter()
                .map(|n| n.flatten())
                .for_each(|x| output.extend(x)),
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! nest {
        // Nest a vector of values
        [$value:tt, $($rest:tt),+] => {
            Nested::Vec(vec![nest![$value], $(nest![$rest]),+])
        };
    
        // Nest deeply nested values
        [[$($values:tt),*]] => {
            Nested::Vec(vec![nest![$($values),*]])
        };
    
        // Nest a single value
        [$value:expr] => {
            Nested::Value($value)
        };
    }

    // #[test]
    // fn test_with_options() {
    //     let nested = nest![Some(1), Some(2), None, [Some(3), Some(4), None, [Some(5)], [[None]]]];
    // }

    #[test]
    fn check_my_work() {
        let nested = nest![[[1], 2, 3], [4], [5, [6, 7], 8], 9];
        assert_eq!(nested.flatten(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    macro_rules! testing {
        // [[$($trees:tt)*]] => {{
        //     println!("[[$(trees:tt)*]] - {}", stringify!($($trees)*));
        //     Nested::Vec(vec![testing![$($trees)*]])
        // }};

        // [$value:expr] => {{
        //     println!("[$value:expr] - {:?}", stringify!($value));
        //     Nested::Value($value)
        // }};

        [$($same_level:tt)* [$($rest:tt)*]] => {{
            println!("{}, {}", stringify!($($same_level)*), stringify!($($rest)*));
            // Nested::Vec(vec![testing![$head], testing![$($rest)*]])
        }};
    }

    #[test]
    fn test_test() {
        let tested = testing![Some(4), Some(5), [Some(6)]];
        println!("{:?}", tested);

        // let nested = nest![[0], 1, [2]];
        // println!("{:?}", nested);
    }
}
