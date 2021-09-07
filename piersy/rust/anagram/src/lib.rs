use std::collections::HashMap;
use std::collections::HashSet;
use std::option;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    // This will find anagrams by inserting the word characters into a haset and then checking that
    // they are matched by the possible anagrams.

    let mut m = HashMap::new();
    let mut result: HashSet<&'a str> = HashSet::new();

    // Add lowercased word chars and count to hashmap.
    for c in word.chars() {
        for l in c.to_lowercase() {
            let count = m.entry(l).or_insert(0);
            *count += 1;
        }
    }
    for w in possible_anagrams {
        let mc = m.clone();
    }
    result
}

fn isAnagram(char_map: &mut HashMap<char, i32>, word: &str) -> bool {
    for c in word.chars() {
        for l in c.to_lowercase() {
            let mut failed = false;
            let o = char_map.get_mut(&l);
            match o {
                None => failed = true,
                Some(v) => {
                    *v -= 1;
                    if *v < 0 {
                        failed = true;
                    }
                }
            }
        }
    }
    true
}

fn foo(param: bool) -> i32 {
    if param {
        return 5;
    };

    println!("param is: {:?}", param);

    6
}
