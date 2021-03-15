//! wordSplitter('seven') -> 0 # first split -> ['sev', 'en'], ['se', 'ven'] X
//! wordSplitter('nine') -> 2 # first split -> ['ni', 'ne'], second split -> ['n', 'i', 'n', 'e'] YAY
//! wordSplitter('fundamentals') -> 2 # first split -> ['fundam', 'entals'], second split -> ['fun', 'dam', 'ent', 'als'], third split -> X
//! wordSplitter('happiest') -> 3 # first split -> ['happ', 'iest'], second split -> ['ha', 'pp', 'ie', 'st'], third split -> ['h', 'a', 'p', 'p', 'i', 'e', 's', 't']

fn word_splitter(word: &str) -> usize {
    let mut splits = 0;
    let mut split_len = word.len();

    while split_len % 2 == 0 {
        split_len /= 2;
        splits += 1;
    }

    splits
}

#[cfg(test)]
mod word_splitter_tests {
    use super::*;

    #[test]
    fn all_tests() {
        assert_eq!(word_splitter("seven"), 0);
        assert_eq!(word_splitter("nine"), 2);
        assert_eq!(word_splitter("fundamentals"), 2);
        assert_eq!(word_splitter("happiest"), 3);
    }
}

fn main() {
    unimplemented!()
}
