use crate::util::read_input;
use regex::Regex;
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

pub fn puzzle1() {
    let map_width = 101;
    let map_height = 103;
    let time = 100;

    let states = parse_input();
    let mut new_states = vec![];

    for state in &states {
        let mut new_px = (state.px + state.vx * (time % map_width)) % map_width;
        while new_px < 0 {
            new_px += map_width;
        }
        while new_px >= map_width {
            new_px -= map_width;
        }

        let mut new_py = (state.py + state.vy * (time % map_height)) % map_height;
        while new_py < 0 {
            new_py += map_height;
        }
        while new_py >= map_height {
            new_py -= map_height;
        }

        new_states.push(Robot {
            px: new_px,
            py: new_py,
            vx: state.vx,
            vy: state.vy,
        })
    }

    for state in &new_states {
        if state.px < 0 || state.px >= map_width {
            panic!();
        }
        if state.py < 0 || state.py >= map_height {
            panic!();
        }
    }

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    for state in new_states {
        if state.px < (map_width - 1) / 2 {
            if state.py < (map_height - 1) / 2 {
                top_left += 1;
            } else if state.py > (map_height - 1) / 2 {
                top_right += 1;
            }
        } else if state.px > (map_width - 1) / 2 {
            if state.py < (map_height - 1) / 2 {
                bottom_left += 1;
            } else if state.py > (map_height - 1) / 2 {
                bottom_right += 1;
            }
        }
    }

    let res = top_left * top_right * bottom_left * bottom_right;

    println!("day 14, puzzle 1: {res}");
}

pub fn puzzle2() {
    let mut robots = parse_input();

    many_time_steps(&mut robots, 42);
    let mut iteration = 42;
    display_robots(&robots);
        println!("{iteration}");
    

    loop {
        iteration += 103;
        many_time_steps(&mut robots, 103);
        display_robots(&robots);
        println!("{iteration}");
        println!("");
        println!("");
        
        sleep(Duration::from_millis(100));
    }

}

fn many_time_steps(robots: &mut Vec<Robot>, steps: i32) {
    for r in robots {
        r.px += r.vx * steps;
        while r.px < 0 {
            r.px += WIDTH as i32;
        }
        while r.px >= WIDTH as i32 {
            r.px -= WIDTH as i32;
        }

        r.py += r.vy * steps;
        while r.py < 0 {
            r.py += HEIGHT as i32;
        }
        while r.py >= HEIGHT as i32 {
            r.py -= HEIGHT as i32;
        }
    }
}

fn display_robots(robots: &Vec<Robot>) {
    let mut map = vec![vec![0; WIDTH as usize]; HEIGHT];

    for robot in robots {
        map[robot.py as usize][robot.px as usize] += 1;
    }

    for row in map {
        for count in row {
            if count > 0 {
                print!("{count}");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn parse_input() -> Vec<Robot> {
    let reader = read_input("input/day_14.txt");
    let mut out = vec![];

    let re = Regex::new(r"p=(.*[0-9]+),(.*[0-9]+) v=(.*[0-9]+),(.*[0-9]+)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let (_, [a, b, c, d]) = re.captures(&line).unwrap().extract();
        // let parts: [&str; 4] = a.1;
        out.push(Robot {
            px: a.to_string().parse().unwrap(),
            py: b.parse().unwrap(),
            vx: c.parse().unwrap(),
            vy: d.parse().unwrap(),
        });
    }

    out
}
