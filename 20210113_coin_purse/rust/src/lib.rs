//! ## Convenience Store
//!
//! Given a total due and an array representing the amount of change in your pocket, determine
//! whether or not you are able to pay for the item. Change will always be represented in the
//! following order: quarters, dimes, nickels, pennies.
//!
//! To illustrate: `changeEnough([25, 20, 5, 0], 4.25)` should yield true, since having 25
//! quarters, 20 dimes, 5 nickels and 0 pennies gives you 6.25 + 2 + .25 + 0 = 8.50.
//!
//! ### Examples
//!
//! ```text
//! changeEnough([2, 100, 0, 0], 14.11) ➞ false
//! changeEnough([0, 0, 20, 5], 0.75) ➞ true
//! changeEnough([30, 40, 20, 5], 12.55) ➞ true
//! changeEnough([10, 0, 0, 50], 3.85) ➞ false
//! changeEnough([1, 0, 5, 219], 19.99) ➞ false
//! ```
//!
//! ### Notes
//!
//! - quarter: 25 cents / $0.25
//! - dime: 10 cents / $0.10
//! - nickel: 5 cents / $0.05
//! - penny: 1 cent / $0.01

use std::collections::HashMap;

/// The `Coin` enum defines the types of coins we might see in our coinpurse
#[derive(PartialEq, Eq, Hash)]
pub enum Coin {
    Quarter,
    Dime,
    Nickel,
    Penny,
}

/// `CoinPurse` will hold the `Coin`s, wrapping a `HashMap` where the keys are the type of coin
/// and the values are the number of coins held
pub struct CoinPurse(HashMap<Coin, u16>);

/// Methods associated with the `CoinPurse` struct, used to create new `CoinPurse`s, add `Coin`s,
/// and tabulate the current total value of the `CoinPurse`
impl CoinPurse {
    pub fn new() -> Self {
        //! Create an empty `CoinPurse`
        CoinPurse(HashMap::new())
    }

    pub fn from_array(arr: [u16; 4]) -> Self {
        //! Create an empty `CoinPurse` and add coins from a sorted array
        let mut new_purse = CoinPurse::new();
        new_purse.add_coin(Coin::Quarter, arr[0]);
        new_purse.add_coin(Coin::Dime, arr[1]);
        new_purse.add_coin(Coin::Nickel, arr[2]);
        new_purse.add_coin(Coin::Penny, arr[3]);
        new_purse
    }

    pub fn get_total(&self) -> f32 {
        //! Iterate over each `Coin` in the `CoinPurse` and sum the value of all coins
        let mut total = 0f32;
        for (coin, amount) in self.0.iter() {
            let famount = f32::from(*amount);
            match coin {
                Coin::Quarter => total += 0.25 * famount,
                Coin::Dime => total += 0.10 * famount,
                Coin::Nickel => total += 0.05 * famount,
                Coin::Penny => total += 0.01 * famount,
            }
        }
        total
    }

    pub fn add_coin(&mut self, coin: Coin, n: u16) -> Option<u16> {
        //! Add a number of `Coin`s to the `CoinPurse` of a given type
        let coins = self.0.get(&coin);
        let new_total = match coins {
            None => n,
            Some(x) => x + n,
        };
        self.0.insert(coin, new_total)
    }
}

/// Given a sorted array indicating the number of each kind of `Coin`, determine whether the sum
/// of the `Coin` values is greater than `amount`.
pub fn change_enough(arr: [u16; 4], amount: f32) -> bool {
    let coin_purse = CoinPurse::from_array(arr);
    coin_purse.get_total() >= amount
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use super::*;

    #[test]
    fn confirm_functionality() {
        assert_eq!(change_enough([ 2, 100,  0,   0], 14.11), false);
        assert_eq!(change_enough([ 0,   0, 20,   5],  0.75), true);
        assert_eq!(change_enough([30,  40, 20,   5], 12.55), true);
        assert_eq!(change_enough([10,   0,  0,  50],  3.85), false);
        assert_eq!(change_enough([ 1,   0,  5, 219], 19.99), false);
    }
}
