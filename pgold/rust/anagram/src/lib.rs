use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn normalise(s: &str) -> Vec<char> {
    s.to_lowercase().chars().sorted().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalised_word = normalise(word);
    HashSet::from_iter(
        possible_anagrams
            .iter()
            .filter(|x| normalise(x) == normalised_word)
            .filter(|x| x.to_lowercase() != word.to_lowercase())
            .cloned(),
    )
}
