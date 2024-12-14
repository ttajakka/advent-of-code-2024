use crate::util::read_input;
use std::collections::HashMap;
use std::io::BufRead;

type RegionID = u32;
type IDPair = (u32, u32);
type Label = u8;
type Map<T> = Vec<Vec<T>>;
type Region = Vec<Coords>;
type Coords = (usize, usize);

pub fn puzzle1() {
    let input_map = parse_input();

    let (id_map, id_region_map) = identify_regions(&input_map);

    let mut areas = count_areas(&id_map);

    let keys = id_region_map.keys().collect::<Vec<&u32>>();

    let (horizontal_fences, vertical_fences) = index_fences(&id_map);
    let fence_lengths = count_fence_lengths(keys,&horizontal_fences, &vertical_fences);

    let mut res = 0;

    for (id, area) in areas.drain() {
        res += area * fence_lengths.get(&id).unwrap();
    }
    
    println!("day 12, puzzle 1: {res}");
    
}

pub fn puzzle2() {

}

fn count_fence_lengths(ids: Vec<&u32>, hor: &Map<IDPair>, vert: &Map<IDPair>) -> HashMap<RegionID, u32> {
    let mut out: HashMap<u32, u32> = HashMap::new();
    for id in ids {
        out.insert(*id, 0);
    }
    out.insert(0,0);

    for row in hor {
        for (left, right) in row {
            if left != right {
                *out.get_mut(&left).unwrap() += 1;
                *out.get_mut(&right).unwrap() += 1
            }
        }
    }

    for row in vert {
        for (top, bottom) in row {
            if top != bottom {
                *out.get_mut(&top).unwrap() += 1;
                *out.get_mut(&bottom).unwrap() += 1
            }
        }
    }

    out
}

fn index_fences(map: &Map<u32>) -> (Map<IDPair>, Map<IDPair>) {
    let h = map.len();
    let w = map[0].len();

    let mut horizontal_fences = vec![vec![(0, 0); w + 1]; h + 1];
    let mut vertical_fences = vec![vec![(0, 0); w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            let id = map[i][j];
            horizontal_fences[i][j].1 = id;
            horizontal_fences[i + 1][j].0 = id;
            vertical_fences[i][j].1 = id;
            vertical_fences[i][j + 1].0 = id;
        }
    }
    (horizontal_fences, vertical_fences)
}

fn count_areas(map: &Map<u32>) -> HashMap<RegionID, u32> {
    let mut out: HashMap<RegionID, u32> = HashMap::new();
    for row in map {
        for id in row {
            if let Some(count) = out.get_mut(id) {
                *count += 1;
            } else {
                out.insert(*id, 1);
            }
        }
    }
    out
}

pub fn print_small_map(map: &Map<RegionID>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", (map[i][j] % 90 + 65) as u8 as char)
        }
        println!()
    }
}

pub fn print_map_from_regions(map: &Map<Label>, regions: &HashMap<RegionID, Region>) {
    let h = map.len();
    let w = map[0].len();
    let mut to_print = vec![vec![46; w]; h];
    for (id, region) in regions.clone().drain() {
        for (i, j) in region {
            to_print[i][j] = id;
        }
    }
    print_small_map(&to_print);
}

fn identify_regions(input_map: &Map<Label>) -> (Map<RegionID>, HashMap<RegionID, Region>) {
    let height = input_map.len();
    let width = input_map[0].len();

    let mut id_map: Map<RegionID> = vec![vec![0; width]; height];
    let mut next_region_id: RegionID = 1;

    let mut id_region_hashmap: HashMap<RegionID, Region> = HashMap::new();

    let i = 0;
    for j in 0..width {
        let label: Label = input_map[i][j];
        // look back along row first:
        if j > 0 && input_map[i][j - 1] == label {
            let id = id_map[i][j - 1];
            id_map[i][j] = id;
            id_region_hashmap.get_mut(&id).unwrap().push((i, j));
        } else {
            let id = next_region_id;
            id_map[i][j] = id;
            id_region_hashmap.insert(id, vec![(i, j)]);
            next_region_id += 1;
        }
    }

    for i in 1..height {
        let mut new_regions: HashMap<RegionID, Region> = HashMap::new();
        for j in 0..width {
            let label: Label = input_map[i][j];
            // look back along row first:
            if j > 0 && input_map[i][j - 1] == label {
                let id = id_map[i][j - 1];
                id_map[i][j] = id;
                new_regions.get_mut(&id).unwrap().push((i, j));
            } else {
                let id = next_region_id;
                id_map[i][j] = id;
                new_regions.insert(id, vec![(i, j)]);
                next_region_id += 1;
            }
            // then look above
            if input_map[i - 1][j] == label && id_map[i - 1][j] != id_map[i][j] {
                let old_id;
                let new_id;
                let squares_to_relabel;
                if id_region_hashmap.contains_key(&id_map[i][j]) {
                    old_id = id_map[i - 1][j];
                    new_id = id_map[i][j];
                    squares_to_relabel = id_region_hashmap.remove(&old_id).unwrap();
                    id_region_hashmap
                        .get_mut(&new_id)
                        .unwrap()
                        .extend(squares_to_relabel.clone());
                } else {
                    old_id = id_map[i][j];
                    new_id = id_map[i - 1][j];
                    squares_to_relabel = new_regions.remove(&old_id).unwrap();
                    if new_regions.contains_key(&new_id) {
                        new_regions
                            .get_mut(&new_id)
                            .unwrap()
                            .extend(squares_to_relabel.clone());
                    } else {
                        new_regions.insert(new_id, squares_to_relabel.clone());
                    }
                }
                for (k, l) in &squares_to_relabel {
                    id_map[*k][*l] = new_id;
                }
            }
        }

        for (k, v) in new_regions.drain() {
            if id_region_hashmap.contains_key(&k) {
                id_region_hashmap.get_mut(&k).unwrap().extend(v);
            } else {
                id_region_hashmap.insert(k, v);
            }
        }
    }

    (id_map, id_region_hashmap)
}

fn parse_input() -> Map<u8> {
    let reader = read_input("input/day_12.txt".to_string());
    let out = reader
        .lines()
        .map(|line| line.unwrap().as_bytes().to_owned())
        .collect();

    out
}
