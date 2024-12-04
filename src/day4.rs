use std::fs::File;
use std::io::{BufRead, BufReader};

const X: u8 = 'X' as u8;
const A: u8 = 'A' as u8;
const M: u8 = 'M' as u8;
const S: u8 = 'S' as u8;

pub fn puzzle1() {
    let input = File::open("input/day_4.txt").unwrap();
    let reader = BufReader::new(input);

    let mut data: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap().into_bytes())
    }

    let mut result = some_xmases(&data);
    let mut data2: Vec<Vec<u8>> = Vec::new();
    for mut row in data {
        row.reverse();
        data2.push(row);
    }
    
    data2.reverse();
    data = data2;

    result += some_xmases(&data);

    println!("{result}");
    
}

fn some_xmases(data: &Vec<Vec<u8>>) -> i32 {
    let h = data.len();
    let w = data[0].len();

    let mut result = 0;

    for i in 0..h {
        for j in 0..w {
            if j < w-3 && hor_xmas(&data, i, j) {
                result += 1;
            }
            if i < h-3 && vert_xmas(&data, i, j) {
                result += 1;
            }
            if i < h-3 && j < w-3 && diag_xmas(&data, i, j) {
                result += 1;
            }
            if i >= 3 && j < w-3 && antidiag_xmas(&data, i, j) {
                result += 1;
            }
        }
    }

    result
}

fn hor_xmas(data: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    return data[row][col] == X
        && data[row][col + 1] == M
        && data[row][col + 2] == A
        && data[row][col + 3] == S;
}

fn vert_xmas(data: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    return data[row][col] == X
        && data[row + 1][col] == M
        && data[row + 2][col] == A
        && data[row + 3][col] == S;
}

fn diag_xmas(data: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    return data[row][col] == X
        && data[row + 1][col + 1] == M
        && data[row + 2][col + 2] == A
        && data[row + 3][col + 3] == S;
}

fn antidiag_xmas(data: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    return data[row][col] == X
        && data[row - 1][col + 1] == M
        && data[row - 2][col + 2] == A
        && data[row - 3][col + 3] == S;
}