/// Yes, I'm defining a struct to represent a digit. Why? Because there's some
/// math later on that _assumes_ each digit is, in fact, a single digit, and if
/// I just use a regular old numeric type then I could have weird edge cases
/// where the value being treated like a single digit is really 10 or greater.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Digit(u8);

/// The `new` method provides value validation for `Digit`s. Now, a Digit must
/// be a value between 0-9. :D
impl Digit {
    pub fn new<T: Into<u8>>(d: T) -> Self {
        let digit: u8 = d.into();
        if digit > 9 {
            panic!("Cannot convert {} into a single digit!", digit);
        }
        Digit(digit)
    }
}

/// Allow us to treat a Digit like a regular old u8
impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        value.0
    }
}

/// Or, we can treat a Digit like a usize. Since a usize is a larger format
/// than a u8, we can losslessly convert any u8 into a usize.
impl From<Digit> for usize {
    fn from(value: Digit) -> Self {
        value.0 as usize
    }
}

/// This struct contains the state necessary to iterate over the digits of
/// a given number.
#[derive(Debug)]
struct DigitsIter<T> {
    remaining: T,
    pow: Option<u32>,
}

/// Implement iterator behavior for the `DigitsIter` struct for u128 numbers.
/// I've opted for u128 numbers because the values in the look-and-say sequence
/// grow quite large quite quickly, so this gives us maximum digits to work with
/// in a numeric value. 39 digits, to be precise.
impl Iterator for DigitsIter<u128> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        let pow = self.pow?;
        let result = self.remaining / 10u128.pow(pow);
        self.remaining -= result * 10u128.pow(pow);
        self.pow = pow.checked_sub(1);
        Some(Digit::new(result as u8))
    }
}

/// Rust traits are a bit like interfaces, in that they allow you to define
/// behavior that will be available for a given type. The `Digits` trait
/// promises that any type implementing it will be able to produce a DigitsIter
/// iterator.
trait Digits<T> {
    fn digits(&self) -> DigitsIter<T>;
}

/// Implement the `Digits` trait for `u128` numbers. Since we're using math to
/// calculate the digits of the number, we need the value remaining and the current
/// power of 10 represented by the value remaining to "peel" digits from the
/// front of the value remaining.
impl Digits<u128> for u128 {
    fn digits(&self) -> DigitsIter<u128> {
        let mut pow = 0;
        while self > &10u128.pow(pow) {
            pow += 1;
        }
        DigitsIter {
            remaining: *self,
            pow: Some(pow.saturating_sub(1)),
        }
    }
}

/// For each item in the run-length encoding, a little custom struct to
/// attach functionality to. Each run-length encoding unit is a pair of the
/// count of the values in a row and the value found. Counts are typically
/// `usize` types in Rust.
#[derive(Debug, Clone, Copy)]
struct RunLengthEncodedUnit<T>(usize, T);

/// And, of course, the run-length encoded unit is also an iterator! This allows
/// us to pull out the count and the value, in order, so long as the value can
/// also be converted to a `usize` like the count.
impl<T> IntoIterator for RunLengthEncodedUnit<T>
where
    T: Into<usize>,
{
    type Item = usize;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1.into()].into_iter()
    }
}

/// A nice generic iterator that takes another iterator and run-length encodes it.
/// Currently restricted to iterators over values that can be trivially copied, to
/// save us from edge cases.
#[derive(Debug)]
struct RunLengthEncodeIter<T, I>
where
    T: PartialEq<T> + Copy,
    I: Iterator<Item = T>,
{
    value: Option<T>,
    iter: I,
}

// Turn the RunLengthEncodeIter<T, I> struct into an actual iterator.
impl<T, I> Iterator for RunLengthEncodeIter<T, I>
where
    T: PartialEq<T> + Copy,
    I: Iterator<Item = T>,
{
    type Item = RunLengthEncodedUnit<T>;

    /// Each time the `next()` method of a RunLengthEncodeIter is called, take
    /// the current value to count (if there is one) and then start taking items
    /// from the enclosed iterator so long as they match the current value. When
    /// `iter` returns something different (either a different value or None),
    /// make that the next value and return a tuple of the current value and
    /// the count.
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.value?;
        let mut count = 1;
        loop {
            match self.iter.next() {
                Some(v) if v == value => count += 1,
                any => {
                    self.value = any;
                    break;
                }
            }
        }
        Some(RunLengthEncodedUnit(count, value))
    }
}

/// Type conversion! If we have a run-length encoding iterator over Digits,
/// then we can convert it into a u128.
impl<I> From<RunLengthEncodeIter<Digit, I>> for u128
where
    I: Iterator<Item = Digit>,
{
    fn from(iter: RunLengthEncodeIter<Digit, I>) -> Self {
        // This approach starts converting values from the iterator to u128
        // values and adding them to the accumulator. This effectively pre-pends
        // digit values to the left end of the u128. Once the accumulator
        // is full, it will contain all the digits from our iterator on the
        // left hand side of the u128, with trailing zeros. To remove the
        // trailing zeros, divide by the remaining power of 10.
        let mut accumulator = 0;
        let mut pow = 39; // 39 digits in a u128
        for n in iter.flatten() {
            pow -= 1;
            accumulator += n as u128 * 10_u128.pow(pow);
        }
        accumulator / 10_u128.pow(pow)
    }
}

/// Trait to allow for chaining other iterators into a RunLengthEncodeIter. This
/// way, we don't need to allocate a new vector or array to run-length encode,
/// we can do it to a streaming iterator of values directly.
trait RunLengthEncode<T, I>
where
    T: PartialEq<T> + Copy,
    I: Iterator<Item = T>,
{
    fn run_length_encode(self) -> RunLengthEncodeIter<T, I>;
}

/// Very generic implementation of RunLengthEncode, for any iterator that
/// yields items of type T. This means that we can run-length encode any
/// iterable with this implementation, so long as we can compare two of
/// the produced values for equality.
impl<T, I> RunLengthEncode<T, I> for I
where
    T: PartialEq<T> + Copy,
    I: Iterator<Item = T>,
{
    /// Take the first item yielded by the enclosed iterator as the first
    /// value to start counting for the first run.
    fn run_length_encode(mut self) -> RunLengthEncodeIter<T, I> {
        let value = self.next();
        RunLengthEncodeIter { value, iter: self }
    }
}

/// Part 1: Write a function to take one value in the look-and-say sequence and
/// produce the next value.
pub fn next_look_and_say_number(number: u128) -> u128 {
    number.digits().run_length_encode().into()
}

/// Part 2: Produce the functionality to take one value in the look-and-say
/// sequence and produce an arbitrary number of the next values in the sequence.
pub fn next_n_look_and_say_numbers(number: u128, n: usize) -> Vec<u128> {
    // The `successors` function takes a value, feeds it to a function,
    // and produces the result in an iterator. Each iteration takes as an
    // argument the value produced in the previous iteration.
    std::iter::successors(Some(number), |x| Some(next_look_and_say_number(*x)))
        .skip(1)
        .take(n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", next_n_look_and_say_numbers(1, 5))
    }
}
