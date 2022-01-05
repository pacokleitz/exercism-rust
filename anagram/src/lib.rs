use std::collections::HashSet;

fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut word1: Vec<char> = word1.to_lowercase().chars().collect();
    let mut word2: Vec<char> = word2.to_lowercase().chars().collect();

    if word1 != word2 {
        word1.sort();
        word2.sort();
        return word1.eq(&word2);
    }
    false
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|w| is_anagram(word, w))
        .cloned()
        .fold(HashSet::new(), |mut hs, w| {
            hs.insert(w);
            hs
        })
}
