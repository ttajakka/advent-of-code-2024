use crate::util::read_input;
use std::io::BufRead;

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
    reader.lines().map(|l| l.unwrap().parse::<u64>().unwrap()).collect()
}