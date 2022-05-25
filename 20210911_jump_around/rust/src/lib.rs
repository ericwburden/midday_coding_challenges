// Starting from 0 on a number line, you would like to make a series of jumps that lead
// to the integer N.
// 
// On the `i`th jump, you may move exactly `i` places to the left or right.
// 
// Find a path with the fewest number of jumps required to get from 0 to N.

use std::collections::VecDeque;

#[allow(dead_code)]
fn jump_to(n: isize) -> Option<isize> {
    jump_math(n);
    jump_deque(n)
}

fn jump_math(n: isize) -> Option<isize> {
    let target = n.abs();
    let mut steps  = 0;
    let mut cumsum = 0;
    loop {
        steps  += 1;
        cumsum += steps;
        if cumsum < target { continue; }
        if (cumsum % 2) == (target % 2) { return Some(steps); }
    }
}


fn jump_deque(n: isize) -> Option<isize> {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0)); 
    while let Some((jumps, current)) = queue.pop_front() {
        if current == n { return Some(jumps); }
        let jump_size = jumps + 1;

        // Add the forward jump
        queue.push_back((jumps + 1, current + jump_size));

        // Add the backward jump
        queue.push_back((jumps + 1, current - jump_size));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_for_small_positive_numbers() {
        assert_eq!(jump_to(2), Some(3));
        assert_eq!(jump_to(3), Some(2));
        assert_eq!(jump_to(10), Some(4));
    }

    #[test]
    fn should_work_for_small_negative_numbers() {
        assert_eq!(jump_to(-2), Some(3));
        assert_eq!(jump_to(-3), Some(2));
        assert_eq!(jump_to(-10), Some(4));
    }

    #[test]
    fn should_work_for_large_positive_numbers() {
        assert_eq!(jump_to(111), Some(17));
        assert_eq!(jump_to(175), Some(21));
        assert_eq!(jump_to(210), Some(20));
    }

    #[test]
    fn should_work_for_large_negative_numbers() {
        assert_eq!(jump_to(-111), Some(17));
        assert_eq!(jump_to(-175), Some(21));
        assert_eq!(jump_to(-210), Some(20));
    }

    #[test]
    fn can_it_withstand_the_mega_jump_test() {
        assert_eq!(jump_to(-154290), Some(555));
    }
}
