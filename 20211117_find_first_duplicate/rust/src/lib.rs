use std::collections::HashMap;
use std::hash::Hash;

// This enum provides a wrapper type for 32-bit signed integers and 
// Strings, so they can be used in the same vector. This basically works
// like a Union type of the two.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum MixedType {
    SignedInteger(i32),
    String(String),
}

impl From<i32> for MixedType {
    fn from(value: i32) -> MixedType {
        MixedType::SignedInteger(value)
    }
}

impl From<&str> for MixedType {
    fn from(value: &str) -> MixedType {
        MixedType::String(value.to_string())
    }
}


fn is_duplicate<T>(value: &T, map: &mut HashMap<T, usize>) -> bool
where
    T: Hash + Eq + Clone,
{
    // Increment the mapped value, starting from 0 if it's not
    // already in the map
    *map.entry(value.to_owned()).or_insert(0) += 1;

    // Return whether or not it's the second time this value
    // has been found
    map.get(value) == Some(&2)
}

fn find_duplicates<T>(arr: Vec<T>) -> Vec<T>
where
    T: Hash + Eq + Clone,
{
    let mut found_map = HashMap::new();

    // For each item in `arr`, chuck it at the `found_map`. If this is the
    // second time it's being added to the map, keep it.
    arr.into_iter().filter(|v| is_duplicate(v, &mut found_map)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_duplicates_where_they_exist() {
        let input = vec![1, 2, 4, 4, 3, 3, 1, 5, 3];
        assert_eq!(find_duplicates(input), vec![4, 3, 1]);
    }

    #[test]
    fn should_find_duplicates_with_mixed_types() {
        let input = vec![
            MixedType::from(1),
            MixedType::from(2),
            MixedType::from(4),
            MixedType::from(4),
            MixedType::from(3),
            MixedType::from(3),
            MixedType::from(1),
            MixedType::from(5),
            MixedType::from(3),
            MixedType::from("5"),
        ];
        assert_eq!(find_duplicates(input), vec![MixedType::from(4), MixedType::from(3), MixedType::from(1)]);
    }

    #[test]
    fn should_return_an_empty_vec_if_no_duplicates_exist() {
        let input = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(find_duplicates(input), vec![]);
    }
}
