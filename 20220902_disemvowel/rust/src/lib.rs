use phf::{phf_set, Set};

static VOWELS: Set<char> = phf_set! {
    'A', 'a', 'E', 'e',
    'I', 'i', 'O', 'o', 
    'U', 'u', 'Y', 'y',
};

pub fn disemvowel_build(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if VOWELS.contains(&c) { continue; }
        out.push(c);
    }
    out
}

pub fn disemvowel_join_str(s: &str) -> String {
    s.chars().filter(|c| !VOWELS.contains(c)).collect::<String>()
}

pub fn disemvowel_join_vec(s: &str) -> String {
    let chars = s.chars().filter(|c| !VOWELS.contains(c)).collect::<Vec<_>>();
    chars.iter().collect::<String>()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_string() {
        let input = "Freedom for the press!";
        let expected = "Frdm fr th prss!";

        let builder_result = disemvowel_build(input);
        assert_eq!(builder_result, expected);

        let join_str_result = disemvowel_join_str(input);
        assert_eq!(join_str_result, expected);
        
        let join_vec_result = disemvowel_join_vec(input);
        assert_eq!(join_vec_result, expected);
    }

    #[test]
    fn test_second_string() {
        let input = "Supreme Leader has funny hair.";
        let expected = "Sprm Ldr hs fnn hr.";

        let builder_result = disemvowel_build(input);
        assert_eq!(builder_result, expected);

        let join_str_result = disemvowel_join_str(input);
        assert_eq!(join_str_result, expected);
        
        let join_vec_result = disemvowel_join_vec(input);
        assert_eq!(join_vec_result, expected);
    }

    #[test]
    fn test_third_string() {
        let input = "I'd like to vote, please.";
        let expected = "'d lk t vt, pls.";

        let builder_result = disemvowel_build(input);
        assert_eq!(builder_result, expected);

        let join_str_result = disemvowel_join_str(input);
        assert_eq!(join_str_result, expected);
        
        let join_vec_result = disemvowel_join_vec(input);
        assert_eq!(join_vec_result, expected);
    }
}
