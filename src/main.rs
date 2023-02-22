use std::env;
use std::io::{self, BufRead};
use std::fs;


struct ActualTree {
    height: usize
}
type TreeMatrix = Vec<Vec<ActualTree>>;


fn main() {
    let path = env::args().nth(1).expect("No file provided!");
    let forest: TreeMatrix = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .into_iter()
        .map(|line| {
            line
                .unwrap()
                .chars()
                .map(|c| ActualTree { height: c.to_string().parse::<usize>().unwrap() })
                .collect::<Vec<ActualTree>>()
        })
        .collect::<TreeMatrix>();

    println!("{} rows parsed.", forest.len());
}
