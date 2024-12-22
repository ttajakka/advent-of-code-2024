use core::panic;
use std::io::BufRead;

use crate::util::read_input;

pub fn puzzle1() {
    let input = parse_input();

    let mut res = 0;

    for (a, l) in input {
        let l: Vec<char> = l.chars().collect();
        println!("{}", l.iter().collect::<String>());

        let l = num_vec_to_arrow_vec(l);
        println!("{}", l.iter().collect::<String>());

        let l = arrow_vec_to_arrow_vec(l);
        println!("{}", l.iter().collect::<String>());

        let l = arrow_vec_to_arrow_vec(l);
        println!("{}", l.iter().collect::<String>());

        println!("{} {a}", l.len());

        res += a * l.len();
    }

    println!("day 21, puzzle 1: {res}");
    
    // let output: Vec<char> = "179A".to_string().chars().collect();
    // println!("{}", output.iter().collect::<String>());

    // let output = num_vec_to_arrow_vec(output);
    // println!("{}", output.iter().collect::<String>());

    // let output = arrow_vec_to_arrow_vec(output);
    // println!("{}", output.iter().collect::<String>());

    // let output = arrow_vec_to_arrow_vec(output);
    // println!("{}", output.iter().collect::<String>());

}

fn num_vec_to_arrow_vec(from: Vec<char>) -> Vec<char> {
    let mut output = num_to_arrows('A', from[0]);
    for i in 0..from.len() - 1 {
        output.append(&mut num_to_arrows(from[i], from[i + 1]))
    }

    output
}

fn num_to_arrows(from: char, to: char) -> Vec<char> {
    let from = num_to_pos(from);
    let to = num_to_pos(to);

    let diff_x = to.1 - from.1;
    let sym_x;
    if diff_x < 0 {
        sym_x = '<';
    } else {
        sym_x = '>';
    }
    let mut syms_x = vec![sym_x; diff_x.abs() as usize];

    let diff_y = to.0 - from.0;
    let sym_y;
    if diff_y < 0 {
        sym_y = 'v';
    } else {
        sym_y = '^';
    }
    let mut syms_y = vec![sym_y; diff_y.abs() as usize];

    let mut out = vec![];
    if from.0 == 0 && to.1 == 1 {
        // if from.1 == 3 {
        //     out.push('<');
        //     syms_x.pop();
        // }
        out.push('^');
        syms_y.pop();
        out.append(&mut syms_x);
        out.append(&mut syms_y);
    // } else if diff_y < 0 {
    //     out.append(&mut syms_y);
    //     out.append(&mut syms_x);
    } else {
        out.append(&mut syms_x);
        out.append(&mut syms_y);
    }

    out.push('A');
    out
}

fn num_to_pos(c: char) -> (i32, i32) {
    match c {
        'A' => (0, 3),
        '0' => (0, 2),
        '1' => (1, 1),
        '2' => (1, 2),
        '3' => (1, 3),
        '4' => (2, 1),
        '5' => (2, 2),
        '6' => (2, 3),
        '7' => (3, 1),
        '8' => (3, 2),
        '9' => (3, 3),
        _ => panic!(),
    }
}

fn arrow_vec_to_arrow_vec(from: Vec<char>) -> Vec<char> {
    let mut output = arrow_to_arrows('A', from[0]);
    for i in 0..from.len() - 1 {
        output.append(&mut arrow_to_arrows(from[i], from[i + 1]));
    }

    output
}

fn arrow_to_arrows(from: char, to: char) -> Vec<char> {
    let out_pre_str = match from {
        'A' => match to {
            'A' => "",
            '^' => "<",
            '>' => "v",
            'v' => "v<",
            '<' => "v<<",
            _ => panic!(),
        },
        '^' => match to {
            'A' => ">",
            '^' => "",
            '>' => "v>",
            'v' => "v",
            '<' => "v<",
            _ => panic!(),
        },
        '>' => match to {
            'A' => "^",
            '^' => "<^",
            '>' => "",
            'v' => "<",
            '<' => "<<",
            _ => panic!(),
        },
        'v' => match to {
            'A' => ">^",
            '^' => "^",
            '>' => ">",
            'v' => "",
            '<' => "<",
            _ => panic!(),
        },
        '<' => match to {
            'A' => ">>^",
            '^' => ">^",
            '>' => ">>",
            'v' => ">",
            '<' => "",
            _ => panic!(),
        },
        _ => panic!(),
    };
    (out_pre_str.to_string() + "A").chars().collect()
}

pub fn parse_input() -> Vec<(usize, String)> {
    let reader = read_input("input/day_21_mock.txt");
    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let a = l.split("A").next().unwrap().parse::<usize>().unwrap();
            (a, l)
        })
        .collect()
}
