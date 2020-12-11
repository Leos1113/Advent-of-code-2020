use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let map: Vec<String> = parse_file();

    let trees_slope_1: usize =  count_trees(&map, 1, 1);
    let trees_slope_2: usize = count_trees(&map, 3, 1);
    let trees_slope_3: usize = count_trees(&map, 5,1 );
    let trees_slope_4: usize = count_trees(&map, 7,1 );
    let trees_slope_5: usize = count_trees(&map, 1, 2);

    let result: usize = trees_slope_1 * trees_slope_2 * trees_slope_3 * trees_slope_4 * trees_slope_5;

    print!("{}", result);
}

fn count_trees(map: &Vec<String>, x_slope: usize, y_slope: usize) -> usize {
    map
        .iter()
        .step_by(y_slope)
        .enumerate()
        .filter(|&(idx, tile)| is_tree(tile, idx * x_slope % tile.len()))
        .count()
}


fn is_tree(tile: &String, pos: usize) -> bool {
    tile.chars().nth(pos).unwrap() == '#'
}



fn parse_file() -> Vec<String> {
    let path = Path::new("input.txt");
    let file = BufReader::new(File::open(&path).unwrap());

    return file.lines()
        .map(|line_result| {
            line_result.unwrap()
        }).collect::<Vec<String>>();
}
