//! # Mid-Day Coding Challenge
//! 
//! Today's challenge brought to you by an interview question asked at Facebook:
//! 
//! Write the code in any language you like. Don't worry if someone has completed the challenge. 
//! Write and post your solution. Daily practice makes progress (and good habits).
//! 
//! ## Future Profits
//! 
//! You work for a hedge fund as a backend developer, particularly on the stock buying/selling 
//! service. Your firm has just had a major breakthrough: they've hired some god-like consultants 
//! to develop an algorithm that is able to reliably predict the stock price at 5 future times. 
//! Unfortunately, god-like consultants demand ungodly hourly rates, and your firm can't afford to 
//! keep them on for long enough to actually put that algorithm to use. That's where you come in...
//! 
//! Given a number of buy/sell actions n and the next five prices for a given stock prices, 
//! identify the maximum profit your firm could make from buying (and then selling) that stock. 
//! For this proof-of-concept phase, don't worry about your starting budget, just assume you can 
//! buy an initial stock whenever is best (thus going into negative profit to start). For this 
//! limited test, you'll only be working with 5 prices and a small n, but your solution should be 
//! extensible to larger values of n and more price predictions (assuming your firm makes enough 
//! profit to pay the consultants to extend their pre-cognitive algorithm).
//! 
//! Some business rules:
//! 
//! - You can only hold one stock at a time, no pumping up the results by buying an arbitrarily 
//!   large number of stocks then selling for marginal profit.
//! - You need to buy a stock before you can sell it.
//! - You need to sell a stock before you can buy another.
//! - It doesn't matter whether you are holding the stock or not at the end of the simulation.
//! - Each time you buy and each time you sell will count against your actions.
//! - You don't need to use all n actions.
//! - Stock prices are given in the order that they occur. You are not a time-traveler and thus 
//!   are unable to buy stocks from the future to sell them in the past.
//! 
//! ## Examples
//! ```ignore
//! n = 4, prices = [5, 2, 4, 0, 1] -> maximum_profit = 3
//! 
//! - Price 1: 5 | Action: None | Actions Left: 4 | Current Profit: 0
//! - Price 2: 2 | Action: Buy  | Actions Left: 3 | Current Profit: -2
//! - Price 3: 4 | Action: Sell | Actions Left: 2 | Current Profit: 2
//! - Price 4: 0 | Action: Buy  | Actions Left: 1 | Current Profit: 2
//! - Price 5: 1 | Action: Sell | Actions Left: 0 | Current Profit: 3
//! ```
//! ```ignore
//! maximum_profit(4, [5, 2, 4, 0, 1]) -> 3
//! maximum_profit(3, [0, 1, 2, 3, 4]) -> 4
//! maximum_profit(4, [5, 8, 9, 3, 3]) -> 4
//! maximum_profit(1, [5, 4, 9, 1, 6]) -> 0
//! maximum_profit(4, [5, 8, 0, 9, 3]) -> 12
//! ```

pub fn maximum_profit(n: u8, prices: &[i32]) -> i32 {
    let max_trades = (n / 2) as usize; // Rust automatically truncates ints
    if max_trades == 0 { return 0; } // Can't do any buy/sell cycles

    let mut potential_profits = Vec::new();  // list of potential trade profits
    let last_idx = prices.len() - 1;  // last index of the `prices` slice
    let mut holding: Option<i32> = None;  // Value of currently held stock, if any

    // For each price in `prices`...
    for (idx, price) in prices.iter().enumerate() {
        // The difference between the current price and the price before it (`lag`) and
        // the price after it (`lead`), with adjustments for the first and last price.
        let lag = if idx == 0 { 0 } else { price - prices[idx - 1] };
        let lead = if idx == last_idx { 0 } else { prices[idx + 1] - price };

        // If the last price and the next price are both higher, buy the stock
        // If the last price and the next price are both lower, sell the stock 
        if lag <= 0 && lead > 0 { holding = Some(*price); }
        if lag > 0 && lead <= 0 && holding.is_some() {
            potential_profits.push(price - holding.unwrap());
            holding = None;
        }
    }

    // Sort potential profits in descending order
    potential_profits.sort_unstable();
    potential_profits.reverse();

    // Total trades is the smaller of either the maximum number of trades allowed
    // or the number of potentially profitable trades
    let total_trades = if potential_profits.len() >= max_trades {
        max_trades
    } else {
        potential_profits.len()
    };

    potential_profits[0..total_trades].iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_macro {
        ($fn_name:ident, $test_invocation:expr, $expected:expr) => {
            #[test]
            fn $fn_name() {
                assert_eq!($test_invocation, $expected);
            }
        };
    }

    test_macro!(test_case1, maximum_profit(4, &[5, 2, 4, 0, 1]), 3);
    test_macro!(test_case2, maximum_profit(3, &[0, 1, 2, 3, 4]), 4);
    test_macro!(test_case3, maximum_profit(4, &[5, 8, 9, 3, 3]), 4);
    test_macro!(test_case4, maximum_profit(1, &[5, 4, 9, 1, 6]), 0);
    test_macro!(test_case5, maximum_profit(4, &[5, 8, 0, 9, 3]), 12);
    test_macro!(test_case6, maximum_profit(3, &[5, 8, 0, 9, 3]), 9);
    test_macro!(test_case7, maximum_profit(3, &[5, 8, 1, 9, 3]), 8);
    test_macro!(test_case8, maximum_profit(4, &[5, 8, 1, 1, 3]), 5);
}
