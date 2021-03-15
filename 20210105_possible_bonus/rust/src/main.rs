fn possible_bonus(x: usize, y: usize) -> bool {
    ((x + 1)..(x + 6)).contains(&y)
}

#[cfg(test)]
mod possible_bonus_tests {
    use super::*;

    #[test]
    fn succes_case_one() {
        assert_eq!(possible_bonus(3, 7), true);
    }

    #[test]
    fn fail_case_one() {
        assert_eq!(possible_bonus(1, 9), false);
    }

    #[test]
    fn fail_case_two() {
        assert_eq!(possible_bonus(5, 3), false)
    }

    #[test]
    fn fail_case_three() {
        assert_eq!(possible_bonus(5, 5), false)
    }
}

fn main() {
    possible_bonus(5, 5); // Because the compiler complains if you don't call the function
}
