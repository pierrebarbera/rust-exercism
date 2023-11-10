use std::collections::{HashSet, HashMap};

fn letter_frequency(word: &str) -> HashMap<char, i32>{
    let mut result = HashMap::new();
    for c in word.chars(){
        let counter = match result.get(&c){
            Some(anything) => anything+1,
            None => 1
        };
        result.insert(c,counter);
    }
    return result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Lifetime specifier 'a
    let mut name = HashSet::new();
    let word_lowercase = word.to_lowercase();
    let word_freq = letter_frequency(&word_lowercase);
    for anagram in possible_anagrams{
        let anagram_lowercase = anagram.to_lowercase();
        let letter_freq = letter_frequency(&anagram_lowercase);
        if word_freq == letter_freq{
            name.insert(*anagram);
        }
    }
    return name
}
