use std::env;
use std::io::{self, BufRead};
use std::fs;
use std::collections::BTreeSet;


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

    // One option would be to evaluate for each tree if it is visible in the grid,
    // this seems very costly though. Instead we count the sets of coordinates 
    // from all four directions that are visible by keeping track of the local 
    // maximum for the row/column and introduce that in a set to avoid double 
    // counting.

    let mut visible_trees: BTreeSet<(usize, usize)> = BTreeSet::new();

    // from the west
    let mut from_west: usize = 0;
    for (i, row) in forest.iter().enumerate() {
        let mut tallest: usize = 0;
        for (j, actual_tree) in row.iter().enumerate() {
            if actual_tree.height > tallest {
                tallest = actual_tree.height;
                visible_trees.insert((i, j));
                from_west += 1;
                print!("{}", actual_tree.height);
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("{} trees visible from the west", from_west);

    // from the east
    let mut from_east: usize = 0;
    for (i, row) in forest.iter().enumerate() {
        let mut tallest: usize = 0;
        for (j, actual_tree) in row.iter().rev().enumerate() {
            if actual_tree.height > tallest {
                tallest = actual_tree.height;
                visible_trees.insert((i, row.len() - j));
                from_east += 1;
            }
        }
    }
    println!("{} trees visible from the east", from_east);

    // from the north
    let mut from_north: usize = 0;
    for (j, _) in forest.first().expect("No rows!").iter().enumerate() {
        let mut tallest: usize = 0;
        for (i, row) in forest.iter().enumerate() {
            let height = row.get(j).unwrap().height;
            if height > tallest {
                tallest = height;
                visible_trees.insert((i, j));
                from_north += 1;
            }
        }
    }
    println!("{} trees visible from the north", from_north);

    // from the south
    let mut from_south: usize = 0;
    for (j, _) in forest.first().expect("No rows!").iter().enumerate() {
        let mut tallest: usize = 0;
        for (i, row) in forest.iter().rev().enumerate() {
            let height = row.get(j).unwrap().height;
            if height > tallest {
                tallest = height;
                visible_trees.insert((forest.len() - i, j));
                from_south += 1;
            }
        }
    }
    println!("{} trees visible from the south", from_south);

    // print results
    println!("Number of visible trees: {}", visible_trees.len());
    // dbg!(visible_trees);

}
