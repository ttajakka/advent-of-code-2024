use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() {
    let input = File::open("input/day_2.txt").unwrap();
    let reader = BufReader::new(input);

    let mut all_reports: Vec<Vec<i64>> = vec![];

    for line in reader.lines() {
        if let Ok(l) = line {
            let report: Vec<i64> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();

            all_reports.push(report);
        }
    }

    let mut safe_count = 0;
    let mut tolerable_count = 0;
    for rep in all_reports {
        if is_report_safe(&rep) {
            safe_count += 1;
        }
        if is_report_tolerable(&rep) {
            tolerable_count += 1;
        }
    }
    println!("day 2, puzzle 1: {safe_count}");
    println!("day 2, puzzle 2: {tolerable_count}");
}

fn is_report_safe(rep: &Vec<i64>) -> bool {
    let mut diffs: Vec<i64> = vec![];
    for i in 0..rep.len() - 1 {
        diffs.push(rep[i + 1] - rep[i]);
    }
    if diffs[0] == 0 {
        return false;
    }

    let min_diff = diffs.iter().min().unwrap();
    let max_diff = diffs.iter().max().unwrap();

    if diffs[0] > 0 {
        if *min_diff > 0 && *max_diff < 4 {
            return true;
        }
    }

    if *min_diff > -4 && *max_diff < 0 {
        return true;
    }

    false
}

fn is_report_tolerable(rep: &Vec<i64>) -> bool {
    if is_report_safe(&rep) {
        return true;
    }

    let n = rep.len();

    for i in 0..n {
        let short_rep = [&rep[0..i], &rep[i + 1..n]].concat();
        if is_report_safe(&short_rep) {
            return true;
        }
    }

    false
}
