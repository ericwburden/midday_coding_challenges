#[allow(dead_code)]
pub fn nth_tribonacci_arr(n: u64, seed: [u64; 3]) -> u64 {
    if n < 3 { return seed[n as usize]; }

    let mut buffer = seed;
    let mut new_num = 0;
    for _ in 0..(n - 2) {
        new_num = buffer[0] + buffer[1] + buffer[2];
        buffer = [buffer[1], buffer[2], new_num];
    }
    new_num
}

#[allow(dead_code)]
pub fn nth_tribonacci_vec(n: u64, seed: [u64; 3]) -> u64 {
    if n < 3 { return seed[n as usize]; }

    let mut buffer = Vec::new();
    let mut new_num = 0;
    seed.iter().for_each(|x| buffer.push(*x));
    for i in 0..(n as usize - 2) {
        new_num = buffer[i] + buffer[i + 1] + buffer[i + 2];
        buffer.push(new_num);
    }
    new_num
}

#[allow(dead_code)]
pub fn nth_tribonacci_inplace(n: u64, seed: [u64; 3]) -> u64 {
    if n < 3 { return seed[n as usize]; }

    let buffer = &mut seed.clone();
    let mut insert = 2;
    let mut new_num = 0;
    for _ in 0..(n - 2) {
        insert = (insert + 1) % 3;
        new_num = buffer[0] + buffer[1] + buffer[2];
        buffer[insert] = new_num;
    }
    new_num
}

#[allow(dead_code)]
pub fn nth_tribonacci_primitive(n: u64, seed: [u64; 3]) -> u64 {
    if n < 3 { return seed[n as usize]; }

    let [mut first, mut second, mut third] = seed;
    let mut new_num = 0;
    for _ in 0..(n - 2) {
        new_num = first + second + third;
        first = second;
        second = third;
        third = new_num;
    }
    new_num
}

#[allow(dead_code)]
pub fn nth_tribonacci_primitive_mod(n: u64, seed: [u64; 3]) -> u64 {
    if n < 3 { return seed[n as usize]; }

    let [mut first, mut second, mut third] = seed;
    let mut insert = 2;
    let mut new_num = 0;
    for _ in 0..(n - 2) {
        insert = (insert + 1) % 3;
        new_num = first + second + third;
        match insert {
            0 => first = new_num,
            1 => second = new_num,
            2 => third = new_num,
            _ => unreachable!(),
        }
    }
    new_num
}

#[allow(dead_code)]
fn nth_tribonacci(n: u64, seed: [u64; 3]) -> u64 {
    nth_tribonacci_primitive(n, seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FUNCTIONS: [fn(u64, [u64; 3]) -> u64; 5] = [
        nth_tribonacci_vec,
        nth_tribonacci_arr,
        nth_tribonacci_primitive,
        nth_tribonacci_inplace,
        nth_tribonacci_primitive_mod,
    ];

    #[test]
    fn test_seed_one() {
        let seed = [1, 1, 1];
        for fun in FUNCTIONS {
            assert_eq!(fun(0, seed), 1);
            assert_eq!(fun(3, seed), 3);
            assert_eq!(fun(7, seed), 31);
            assert_eq!(fun(10, seed), 193);
            assert_eq!(fun(50, seed), 7440059097409);
        }
    }

    #[test]
    fn test_seed_two() {
        let seed = [0, 0, 1];
        for fun in FUNCTIONS {
            assert_eq!(fun(2, seed), 1);
            assert_eq!(fun(3, seed), 1);
            assert_eq!(fun(8, seed), 24);
            assert_eq!(fun(10, seed), 81);
            assert_eq!(fun(50, seed), 3122171529233);
        }
    }
}
