use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let map: Vec<String> = parse_file();

    let trees =  count_trees(&map, 3, 1);

    print!("{}", trees);
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
