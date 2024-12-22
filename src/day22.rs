use crate::util::read_input;
use std::{collections::HashMap, io::BufRead};

const MOD: u64 = 16777216;

pub fn puzzle1() {
    let input = parse_input();
    let mut res = 0;
    for i in input {
        let secret = generate_nth_secret(i, 2000);
        res += secret;
    }

    println!("day 22, puzzle 1: {res}");
}

pub fn puzzle2() {
    let input = parse_input();
    // let input = vec![123];
    let price_lists = generate_price_lists(input);
    let diff_lists = generate_diff_lists(&price_lists);

    let mut sell_sequences: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();
    for i in 0..price_lists.len() {
        let mut local_sell_sequences: HashMap<(i8, i8, i8, i8), i8> = HashMap::new();
        let prices = &price_lists[i];
        let diffs = &diff_lists[i];
        for j in 0..diffs.len()-3 {
            let key = (diffs[j], diffs[j+1], diffs[j+2], diffs[j+3]);
            if local_sell_sequences.contains_key(&key) {
                continue;
                
            } else {
                local_sell_sequences.insert(key, prices[j+4]);
            }
        }
        
        for (key, value) in local_sell_sequences.drain() {
            let e = sell_sequences.entry(key).or_default();
            *e += value as i64;
        }
    }

    let res = sell_sequences.values().max().unwrap();

    println!("day 22, puzzle 2: {res}");
    

}

fn generate_diff_lists(price_lists: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut out = vec![vec![0; price_lists[0].len() - 1]; price_lists.len()];

    for i in 0..price_lists.len() {
        let list = &price_lists[i];
        for j in 0..list.len() - 1 {
            out[i][j] = list[j + 1] - list[j];
        }
    }

    out
}

fn generate_price_lists(input: Vec<u64>) -> Vec<Vec<i8>> {
    let mut price_lists: Vec<Vec<i8>> = vec![];
    for i in input {
        let mut prices = vec![];
        let mut secret = i;
        for _ in 0..2000 {
            prices.push((secret % 10) as i8);
            secret = generate_next_secret(secret);
        }
        prices.push((secret % 10) as i8);
        price_lists.push(prices);
    }
    price_lists
}

fn generate_nth_secret(mut secret: u64, n: u64) -> u64 {
    for _ in 0..n {
        secret = generate_next_secret(secret);
    }
    secret
}

fn generate_next_secret(secret: u64) -> u64 {
    let result = secret * 64;
    let secret = mix_and_prune(secret, result);

    let result = secret / 32;
    let secret = mix_and_prune(secret, result);

    let result = secret * 2048;
    let secret = mix_and_prune(secret, result);

    secret
}

fn mix_and_prune(secret: u64, result: u64) -> u64 {
    (secret ^ result) % MOD
}

fn parse_input() -> Vec<u64> {
    let reader = read_input("input/day_22.txt");
    reader
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect()
}
