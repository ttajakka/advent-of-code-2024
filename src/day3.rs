use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn day3() {
    let input = File::open("input/day_3.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    let res = calc_line(&input);
    println!("day 3, puzzle 1: {res}");

    let res = puzzle_2(&input);
    println!("day 3, puzzle 2: {res}");
}

fn calc_line(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").unwrap();

    let mut res = 0;
    for (_, [m, n]) in re.captures_iter(&input).map(|c| c.extract()) {
        res += (m.parse::<i32>().unwrap()) * (n.parse::<i32>().unwrap());
    }
    res
}

fn puzzle_2(input: &str) -> i32 {
    let mut res = 0;
    for line in input.split("do()") {
        let enabled = line.split("don't()").next().unwrap();
        res += calc_line(enabled);
    }
    return res;
}
