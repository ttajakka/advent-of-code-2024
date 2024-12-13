use crate::util::read_input;
use std::collections::HashMap;
use std::io::BufRead;

type RegionID = u32;
type Label = u8;
type Map<T> = Vec<Vec<T>>;
type Region = Vec<Coords>;
type Coords = (usize, usize);

pub fn puzzle1() {
    let input_map = parse_input();

    let (id_map, mut id_region_map) = identify_regions(&input_map);
    // println!("{id_map:?}");
    // println!("{id_region_map:?}");
    let mut sum = 0;
    for (k, v) in id_region_map.drain() {
        println!("{k}: {}", v.len());
        sum += v.len();
    }
    println!("{sum}");
    

    print_small_map(&id_map);
}

fn print_small_map(map: &Map<RegionID>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", (map[i][j] % 90 + 65) as u8 as char)
        }
        println!()
    }
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
            println!("{i} {j}");
            
            println!("top: {:?}", new_regions);
            
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
                } else {
                    old_id = id_map[i][j];
                    new_id = id_map[i - 1][j];
                    squares_to_relabel = new_regions.remove(&old_id).unwrap();
                    new_regions.insert(new_id, squares_to_relabel.clone());
                }
                for (k, l) in &squares_to_relabel {
                    id_map[*k][*l] = new_id;
                }
                id_region_hashmap
                    .get_mut(&new_id)
                    .unwrap()
                    .extend(squares_to_relabel);
            }
            // println!("{i} {j}: {:?}", id_region_hashmap);
            // println!("{i} {j} {:?}", id_region_hashmap.get(&1));
            println!("bottom: {:?}", new_regions);

            
        }

        for (k, v) in new_regions.drain() {
            if id_region_hashmap.contains_key(&k) {
                id_region_hashmap.get_mut(&k).unwrap().extend(v);
            } else {
                id_region_hashmap.insert(k, v);
            }
        }
        println!("{:?}", id_region_hashmap);
        
    }

    (id_map, id_region_hashmap)
}

fn parse_input() -> Map<u8> {
    let reader = read_input("input/day_12_mock.txt".to_string());
    let out = reader
        .lines()
        .map(|line| line.unwrap().as_bytes().to_owned())
        .collect();

    out
}
