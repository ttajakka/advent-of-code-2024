use std::{cmp::Ordering, collections::HashMap, io::BufRead};

use crate::util::read_input;

pub fn puzzle1() {
    let graph = parse_input();

    let mut triangles: Vec<(String, String, String)> = vec![];

    let mut keys = graph.keys().to_owned().collect::<Vec<_>>();
    keys.sort();

    for i in 0..keys.len() {
        let edges = graph.get(keys[i]).unwrap();
        for j in 0..edges.len() - 1 {
            if !graph.contains_key(&edges[j]) {
                continue;
            }
            for k in j..edges.len() {
                if graph.get(&edges[j]).unwrap().contains(&edges[k]) {
                    triangles.push((
                        keys[i].to_string(),
                        edges[j].to_string(),
                        edges[k].to_string(),
                    ));
                }
            }
        }
    }

    let mut res = 0;

    for t in &triangles {
        if t.0.starts_with("t") || t.1.starts_with("t") || t.2.starts_with("t") {
            res += 1;
        }
    }

    println!("day 23, puzzle 1: {}", res);
}

fn parse_input() -> HashMap<String, Vec<String>> {
    let reader = read_input("input/day_23.txt");
    let mut edges: Vec<(String, String)> = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut parts = l.split("-");
            let a = parts.next().unwrap().to_string();
            let b = parts.next().unwrap().to_string();
            match a.cmp(&b) {
                Ordering::Less => (a, b),
                Ordering::Greater => (b, a),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>();

    edges.sort_by(|a, b| {
        let cmp_firsts = a.0.cmp(&b.0);
        if cmp_firsts == Ordering::Equal {
            return a.1.cmp(&a.1);
        }
        return cmp_firsts;
    });

    let mut graph = HashMap::new();
    for edge in &edges {
        let (a, b) = edge;
        if graph.contains_key(a) {
            let v: &mut Vec<String> = graph.get_mut(a).unwrap();
            v.push(b.to_string())
        } else {
            graph.insert(a.to_string(), vec![b.to_string()]);
        };
    }

    for (_, v) in graph.iter_mut() {
        v.sort();
    }

    graph
}
