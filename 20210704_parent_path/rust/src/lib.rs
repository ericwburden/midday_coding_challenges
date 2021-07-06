// You are given a data table that looks like this:
//
// | id |         name         | parentID |
// |:--:|:--------------------:|:--------:|
// |  1 |        'Home'        |   null   |
// |  2 |      'Products'      |   null   |
// |  3 |      'Consoles'      |     2    |
// |  4 |         'PS4'        |     3    |
// |  5 |         'PS5'        |     3    |
// |  6 |     'Golf-Clubs'     |     2    |
// |  7 | 'Taylor-Made-9-Iron' |     6    |
//
// create a function that takes in an Id number and outputs all related names as a URL
// path. Relative data will be based off of the parentId.

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

// Sets up the input data structure as a HashMap where the `id` is the key, and the
// value is a tuple of `(name, parentID)`. We're using `0` instead of `null` because
// Rust doesn't actually allow `null` values. We could use an `Option<u8>` here, but
// that's overcomplicated for this example.
lazy_static! {
    static ref PRODUCTS: HashMap<u8, (&'static str, u8)> = [
        (1, ("Home", 0)),
        (2, ("Products", 0)),
        (3, ("Consoles", 2)),
        (4, ("PS4", 3)),
        (5, ("PS5", 3)),
        (6, ("Golf-Clubs", 2)),
        (7, ("Taylor-Made-9-Iron", 6))
    ].iter().cloned().collect();
}

pub fn relative_url(id: u8) -> String {
    let mut curr_id = id;
    let mut out: Vec<_> = Vec::new();

    while let Some((name, parent_id)) = PRODUCTS.get(&curr_id) {
        out.push(name.to_string());
        curr_id = *parent_id;
    }
    out.reverse();
    out.into_iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = relative_url(7);
        let expected = String::from("products/golf-clubs/taylor-made-9-iron");
        assert_eq!(result, expected);
    }
}
