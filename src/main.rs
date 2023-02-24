use std::env;
use std::io::{self, BufRead};
use std::fs;
use std::collections::BTreeSet;


struct ActualTree {
    height: isize
}
type TreeMatrix = Vec<Vec<ActualTree>>;


/// Counts trees visible from outside the grid.
///
/// One option would be to evaluate for each tree if it is visible in the grid,
/// this seems very costly though. Instead we count the sets of coordinates 
/// from all four directions that are visible by keeping track of the local 
/// maximum for the row/column and introduce that in a set to avoid double 
/// counting.
fn get_visible_trees(forest: &TreeMatrix) -> usize {

    let mut visible_trees: BTreeSet<(usize, usize)> = BTreeSet::new();

    // from the west 
    let mut from_west: usize = 0;
    for i in 0..forest.len() {
        let mut tallest: isize = -1;
        for j in 0..forest.get(i).unwrap().len() {
            let actual_tree = forest.get(i).unwrap().get(j).unwrap();
            if actual_tree.height > tallest {
                tallest = actual_tree.height;
                visible_trees.insert((i, j));
                from_west += 1;
            }
        }
    }
    println!("{} trees visible from the west", from_west);
    
    // from the east 
    let mut from_east: usize = 0;
    for i in 0..forest.len() {
        let mut tallest: isize = -1;
        for j in (0..forest.get(i).unwrap().len()).rev() {
            let actual_tree = forest.get(i).unwrap().get(j).unwrap();
            if actual_tree.height > tallest {
                tallest = actual_tree.height;
                visible_trees.insert((i, j));
                from_east += 1;
            }
        }
    }
    println!("{} trees visible from the east", from_east);

    // from the north 
    let mut from_north: usize = 0;
    for j in 0..forest.first().unwrap().len() {
        let mut tallest: isize = -1;
        for i in 0..forest.len() {
            let actual_tree = forest.get(i).unwrap().get(j).unwrap();
            if actual_tree.height > tallest {
                tallest = actual_tree.height;
                visible_trees.insert((i, j));
                from_north += 1;
            }
        }
    }
    println!("{} trees visible from the north", from_north);

    // from the south 
    let mut from_south: usize = 0;
    for j in 0..forest.first().unwrap().len() {
        let mut tallest: isize = -1;
        for i in (0..forest.len()).rev() {
            let actual_tree = forest.get(i).unwrap().get(j).unwrap();
            if actual_tree.height > tallest {
                tallest = actual_tree.height;
                visible_trees.insert((i, j));
                from_south += 1;
            }
        }
    }
    println!("{} trees visible from the south", from_south);

    visible_trees.len()

}


/// Returns the scenic score for a set of coordinates in a tree grid
fn get_score(forest: &TreeMatrix, i: &usize, j: &usize) -> usize {
    let height = forest.get(*i).unwrap().get(*j).unwrap().height;
    let size_y = forest.len();
    let size_x = forest.first().unwrap().len();

    let mut count_up: usize = 0;
    for up in (0..*i).rev() {
        let horizon = forest.get(up).unwrap().get(*j).unwrap().height;
        if height >= horizon {
            count_up += 1;
        }
        if height <= horizon {
            break;
        }
    }

    let mut count_down: usize = 0;
    for down in *i+1..size_y {
        let horizon = forest.get(down).unwrap().get(*j).unwrap().height;
        if height >= horizon {
            count_down += 1;
        }
        if height <= horizon {
            break;
        }
    }

    let mut count_left: usize = 0;
    for left in (0..*j).rev() {
        let horizon = forest.get(*i).unwrap().get(left).unwrap().height;
        if height >= horizon {
            count_left += 1;
        }
        if height <= horizon {
            break;
        }
    }
    
    let mut count_right: usize = 0;
    for right in *j+1..size_x {
        let horizon = forest.get(*i).unwrap().get(right).unwrap().height;
        if height >= horizon {
            count_right += 1;
        }
        if height <= horizon {
            break;
        }
    }
    
    count_up * count_down * count_left * count_right
}


/// Returns the best scenic score for the tree grid
fn get_best_tree(forest: &TreeMatrix) -> usize {
    let size_y = forest.len();
    let size_x = forest.first().unwrap().len();

    let mut max_score: usize = 0;

    for i in 0..size_y {
        for j in 0..size_x {
            let score = get_score(forest, &i, &j);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}


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
                .map(|c| ActualTree { height: c.to_string().parse::<isize>().unwrap() })
                .collect::<Vec<ActualTree>>()
        })
        .collect::<TreeMatrix>();

    println!("{} rows parsed.", forest.len());

    let part_one = get_visible_trees(&forest);
    let part_two = get_best_tree(&forest);

    // print results
    println!("Number of visible trees: {}", part_one);
    println!("Best scenic score: {}", part_two);

}
