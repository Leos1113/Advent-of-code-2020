use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Can't open input.txt");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut numbers: Vec<i32> = Vec::new();
    for line in contents.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }
    numbers.sort();
    let mut index = 0;
    let mut little = numbers[index];
    let mut bigger = numbers.last().unwrap();

    loop {
        if (little + bigger) > 2020 {
            numbers.pop();
            bigger = numbers.last().unwrap();
        } else if little + bigger < 2020 {
            index += 1;
            little = numbers[index];
        } else {
            let result = little * bigger;
            println!("{}", result);
            break;
        }
    }
}
