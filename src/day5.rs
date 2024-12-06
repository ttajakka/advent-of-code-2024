use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> BufReader<File> {
    let input = File::open("input/day_5.txt").unwrap();
    let reader = BufReader::new(input);

    reader
}

pub fn puzzle1() {
    let bufreader = read_input();
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut read_rules = true;
    for line in bufreader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            read_rules = false;
            continue;
        }
        if read_rules {
            rules.push(parse_rule(line))
        } else {
            updates.push(parse_update(line));
        }
    }
    let mut result = 0;

    for update in updates {
        if update_in_order(&update, &rules) {
            result += update[update.len() / 2];
        }
    }
    println!("day 5, puzzle 1: {result}");
}

fn parse_rule(line: String) -> (i32, i32) {
    let rule_re = Regex::new(r"^([0-9]+)\|([0-9]+)$").unwrap();
    let (_, [a, b]) = rule_re.captures(&line).unwrap().extract();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn parse_update(line: String) -> Vec<i32> {
    line.split(",").map(|a| a.parse::<i32>().unwrap()).collect()
}

fn update_in_order(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for i in 0..update.len() - 1 {
        for j in i..update.len() {
            if rules.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}
