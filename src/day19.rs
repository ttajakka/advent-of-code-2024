use crate::util::read_input;
use std::{collections::HashMap, io::Read};

type Alphabet = HashMap<usize, Vec<String>>;

pub fn puzzle1() {
    let (alphabet, words) = parse_input();

    let alphabet = filter_alphabet(alphabet);

    let mut res = 0;

    for i in 0..words.len() {
        let clone = words[i].clone();
        if word_is_possible(clone, &alphabet, 0) {
            res += 1;
        }
    }


    println!("day 19, puzzle 1: {res}");
}

fn filter_alphabet(alphabet: Alphabet) -> Alphabet {
    let mut filtered: HashMap<usize, Vec<String>> = HashMap::new();
    let mut keys = alphabet.keys().collect::<Vec<_>>();
    keys.sort();
    
    for k in keys {
        filtered.insert(*k, vec![]);
        for word in alphabet.get(k).unwrap() {
            if !word_is_possible(word.clone(), &filtered, 0) {
                let entry = filtered.get_mut(k).unwrap();
                entry.push(word.clone())
            }
        }
    }
    filtered
} 

fn word_is_possible(word: String, alphabet: &Alphabet, level: usize) -> bool {
    if word.len() == 0 {
        return true;
    }

    for key in alphabet.keys() {
        let alphas = alphabet.get(key).unwrap();
        let mut starts_with = false;
        for a in alphas {
            if word.starts_with(a) {
                starts_with = true;
                break;
            }
        }
        if starts_with {
            let clone = word.clone().split_off(*key);
            if word_is_possible(clone, alphabet, level + 1) {
                return true;
            }
        }
    }
    false
}

fn parse_input() -> (HashMap<usize, Vec<String>>, Vec<String>) {
    let mut reader = read_input("input/day_19.txt");
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    let mut parts = buf.split("\n\n");
    let mut alphabet = HashMap::new();
    for a in parts.next().unwrap().split(", ") {
        if alphabet.contains_key(&a.len()) {
            let entry: &mut Vec<String> = alphabet.get_mut(&a.len()).unwrap();
            entry.push(a.to_string());
        } else {
            alphabet.insert(a.len(), vec![a.to_string()]);
        }
    }

    let words = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    (alphabet, words)
}
