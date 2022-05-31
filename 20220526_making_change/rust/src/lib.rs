use std::collections::HashSet;

pub fn make_change_dfs(n: u64, denoms: &[u64]) -> u64 {
    let mut stack = vec![(Vec::new(), n)];
    let mut ways = HashSet::new();

    while !stack.is_empty() {
        let (used, remaining ) = stack.pop().unwrap();
        for coin in denoms {
            if coin > &remaining { continue; }

            let mut now_used = used.clone();
            now_used.push(*coin);
            now_used.sort_unstable(); // So that [1, 5] == [5, 1]
            if coin == &remaining {
                ways.insert(now_used); 
            } else {
                stack.push((now_used, remaining - *coin));
            }   
        }
    }
    ways.len() as u64
}

pub fn make_change_rec(n: u64, denoms: &[u64]) -> u64 {
    if n == 0 { return 0; } // Need an exception for the first call

    fn inner(n: u64, denoms: &[u64]) -> u64 {
        if n == 0 { return 1; }
        let mut ways = 0;
        for (idx, coin) in denoms.iter().enumerate() {
            if coin > &n { continue; }
            ways += inner(n - coin, &denoms[idx..]);
        }
        ways
    }

    inner(n, denoms)
}

pub fn make_change_dp(n: u64, denoms: &[u64]) -> u64 {
    if n == 0 { return 0; }
    let mut ways = vec![0; n as usize + 1];
    ways[0] = 1;

    for coin in denoms {
        for way in 0..=n as usize {
            if *coin > way as u64 { continue; }
            ways[way] += ways[way - *coin as usize];
        }
    }

    ways[n as usize]
}

pub fn make_change(n: u64, denoms: &[u64]) -> u64 {
    make_change_dp(n, denoms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(make_change( 6, &[1, 5]),          2);
        assert_eq!(make_change( 0, &[2, 3, 4, 7]),    0);
        assert_eq!(make_change( 0, &[5]),             0);
        assert_eq!(make_change( 7, &[2, 4]),          0);
        assert_eq!(make_change( 4, &[1, 5, 10, 25]),  1);
        assert_eq!(make_change( 5, &[1, 5, 10, 25]),  2);
        assert_eq!(make_change(10, &[1, 5, 10, 25]),  4);
        assert_eq!(make_change(25, &[1, 5, 10, 25]), 13);
    }
}
