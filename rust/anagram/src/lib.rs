use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            let a = word.to_lowercase();
            let b = possible_anagram.to_lowercase();

            if a.len() == b.len() && a != b {
                let mut letters_in_a = a.chars().collect::<Vec<char>>();
                let mut letters_in_b = b.chars().collect::<Vec<char>>();

                letters_in_a.sort();
                letters_in_b.sort();

                letters_in_a == letters_in_b
            } else {
                false
            }
        })
        .copied()
        .collect()
}
