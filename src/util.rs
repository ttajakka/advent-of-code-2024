use std::fs::File;
use std::io::BufReader;

pub fn read_input(path: &str) -> BufReader<File> {
    let input = File::open(path).unwrap();
    let reader = BufReader::new(input);

    reader
}
