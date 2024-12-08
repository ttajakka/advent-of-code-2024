use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

pub fn puzzle1() {
    let (height, width) = get_dims();
    let antennas = parse_input();
    let mut unique_antinodes = vec![];
    let antinodes: Vec<Vec<Point>> = antennas
        .into_values()
        .map(|a| antennas_to_antinodes(a))
        .collect();
    for list in antinodes {
        for a in list {
            if antinode_in_box(&a, height, width) && !unique_antinodes.contains(&a) {
                unique_antinodes.push(a)
            }
        }
    }

    print_antinodes(&unique_antinodes, height, width);

    println!("day 8, puzzle 1: {}", unique_antinodes.len())
}

pub fn puzzle2() {
    let (height, width) = get_dims();
    let antennas = parse_input();
    let mut unique_antinodes = vec![];
    let antinodes: Vec<Vec<Point>> = antennas
        .into_values()
        .map(|a| antennas_to_antinodes_2(a, height, width))
        .collect();
    for list in antinodes {
        for a in list {
            if antinode_in_box(&a, height, width) && !unique_antinodes.contains(&a) {
                unique_antinodes.push(a)
            }
        }
    }

    print_antinodes(&unique_antinodes, height, width);

    println!("day 8, puzzle 1: {}", unique_antinodes.len())
}

fn print_antinodes(antinodes: &Vec<Point>, height: usize, width: usize) {
    let mut canvas = vec![vec!['.'; width]; height];
    for a in antinodes {
        canvas[a.x as usize][a.y as usize] = '#';
    }
    for i in 0..height {
        for j in 0..width {
            print!("{}", canvas[i][j])
        }
        println!("");
    }
}

fn antinode_in_box(antinode: &Point, height: usize, width: usize) -> bool {
    if antinode.x < 0 || antinode.x >= height as i32 {
        return false;
    }
    if antinode.y < 0 || antinode.y >= width as i32 {
        return false;
    }

    true
}

fn antennas_to_antinodes(antennas: Vec<Point>) -> Vec<Point> {
    let mut antinodes = vec![];
    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let tail = &antennas[i];
            let head = &antennas[j];
            let dx = head.x - tail.x;
            let dy = head.y - tail.y;
            antinodes.push(Point {
                x: tail.x - dx,
                y: tail.y - dy,
            });
            antinodes.push(Point {
                x: head.x + dx,
                y: head.y + dy,
            })
        }
    }
    antinodes
}

fn antennas_to_antinodes_2(antennas: Vec<Point>, height: usize, width: usize) -> Vec<Point> {
    let mut antinodes = vec![];
    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let tail = &antennas[i];
            let head = &antennas[j];
            let dx = head.x - tail.x;
            let dy = head.y - tail.y;

            let mut looper = Point {
                x: tail.x,
                y: tail.y,
            };
            while antinode_in_box(&looper, height, width) {
                antinodes.push(Point {
                    x: looper.x,
                    y: looper.y,
                });
                looper = Point {
                    x: looper.x - dx,
                    y: looper.y - dy,
                };
            }

            looper = Point {
                x: head.x,
                y: head.y,
            };
            while antinode_in_box(&looper, height, width) {
                antinodes.push(Point {
                    x: looper.x,
                    y: looper.y,
                });
                looper = Point {
                    x: looper.x + dx,
                    y: looper.y + dy,
                };
            }
        }
    }
    antinodes
}

fn parse_input() -> HashMap<u8, Vec<Point>> {
    let mut antennas: HashMap<u8, Vec<Point>> = HashMap::new();
    let lines: Vec<Vec<u8>> = read_input()
        .lines()
        .map(|a| a.unwrap().as_bytes().to_vec())
        .collect();
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            let key = lines[x][y];
            if key == '.' as u8 {
                continue;
            }
            let a = antennas.entry(key).or_insert(vec![]);
            a.push(Point {
                x: x as i32,
                y: y as i32,
            })
        }
    }

    antennas
}

fn get_dims() -> (usize, usize) {
    let lines: Vec<String> = read_input().lines().map(|a| a.unwrap()).collect();
    (lines.len(), lines[0].len())
}

fn read_input() -> BufReader<File> {
    let input = File::open("input/day_8.txt").unwrap();
    let reader = BufReader::new(input);

    reader
}
