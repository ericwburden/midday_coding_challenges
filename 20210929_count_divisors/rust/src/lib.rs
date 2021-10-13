fn divisors(n: u32) -> u32 {
    if n == 1 { return 1; }
    let (mut count, mut current, mut max) = (0, 1, n);

    while current < max {
        if n % current == 0 {
            max = n / current;
            count += if current == max { 1 } else { 2 }
        }
        current += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: u32,
        expected: u32,
    }

    impl TestCase {
        fn new(input: u32, expected: u32) -> Self {
            TestCase { input, expected }
        }

        fn test(&self) {
            let result = divisors(self.input);
            if result != self.expected {
                panic!("Checked {}, expected {}, got {}", self.input, self.expected, result);
            }
        }
    }

    #[test]
    fn it_works() {
        let test_cases = [
            TestCase::new(      1, 1),
            TestCase::new(      4, 3),
            TestCase::new(      5, 2),
            TestCase::new(     12, 6),
            TestCase::new(     30, 8),
            TestCase::new(     81, 5),
            TestCase::new(500_000, 42),
        ];
        test_cases.iter().for_each(|x| x.test());
    }
}
