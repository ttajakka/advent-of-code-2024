use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input = File::open("input/day_1.txt").unwrap();
    let reader = BufReader::new(input);

    let mut nums1: Vec<i64> = vec![];
    let mut nums2: Vec<i64> = vec![];

    for line in reader.lines() {
        if let Ok(l) = line {
            let mut parts = l.split("   ");
            let part1: i64 = parts.next().unwrap().parse().unwrap();
            let part2: i64 = parts.next().unwrap().parse().unwrap();
            
            nums1.push(part1);
            nums2.push(part2);
        }
    }

    nums1.sort();
    nums2.sort();

    let mut res: i64 = 0;

    for i in 0..nums1.len() {
        println!("{i}");
        
        res += (nums1[i] - nums2[i]).abs();
    }
    
    println!("{res}");


    let mut frequencies: HashMap<i64, i64> = HashMap::new();
    for i in nums2 {
        let freq = frequencies.entry(i).or_insert(0);
        *freq += 1;
    }

    let mut res: i64 = 0;
    for i in nums1 {
        res += i * frequencies.get(&i).copied().unwrap_or(0);
    }

    println!("{res}");
    
    
}
