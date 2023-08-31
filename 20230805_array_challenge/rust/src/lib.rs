#![allow(dead_code)]

struct FibonacciIterator {
    values: [usize; 3],
    index: usize,
}

impl FibonacciIterator {
    fn new() -> Self {
        let values = [1, 1, 0];
        let index = 2;
        Self { values, index }
    }
}

impl Iterator for FibonacciIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.values[self.index];
        let index2 = (self.index + 1) % 3;
        let index3 = (index2 + 1) % 3;
        self.values[self.index] = self.values[index2] + self.values[index3];
        self.index = index2;
        return Some(result);
    }
}

fn array_challenge(numbers: &[usize]) -> usize {
    let total: usize = numbers.iter().sum();
    let next_fib: usize = FibonacciIterator::new()
        .skip_while(|n| *n < total)
        .next()
        .unwrap();
    next_fib - total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_iterator_produces_fib_numbers() {
        let mut fibiter = FibonacciIterator::new();
        assert_eq!(fibiter.next(), Some(0));
        assert_eq!(fibiter.next(), Some(1));
        assert_eq!(fibiter.next(), Some(1));
        assert_eq!(fibiter.next(), Some(2));
        assert_eq!(fibiter.next(), Some(3));
        assert_eq!(fibiter.next(), Some(5));
        assert_eq!(fibiter.next(), Some(8));
        assert_eq!(fibiter.next(), Some(13));
        assert_eq!(fibiter.next(), Some(21));
        assert_eq!(fibiter.next(), Some(34));
        assert_eq!(fibiter.next(), Some(55));
    }

    #[test]
    fn array_challenge_complete() {
        assert_eq!(array_challenge(&[5, 2, 1]), 0);
        assert_eq!(array_challenge(&[1, 20, 2, 5]), 6);
        assert_eq!(
            array_challenge(&[1111, 22222, 33333, 12345, 4321, 1234, 321, 123]),
            15
        );
    }
}
