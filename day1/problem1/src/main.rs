use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let content = read_file("input.txt");

    let mut input_numbers = content_lines_to_integer(content);

    input_numbers.sort();

    println!("{}", multiply_the_two_numbers_that_sum_2020(input_numbers));
}

fn read_file(filename: &str) -> String {
    let file = File::open(filename).expect("Can not open the file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}

fn content_lines_to_integer(content: String) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for line in content.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    return numbers;
}

fn multiply_the_two_numbers_that_sum_2020(mut numbers: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut little = numbers[index];
    let mut bigger = numbers.last().unwrap();

    return loop {
        if little + bigger > 2020 {
            numbers.pop();
            bigger = numbers.last().unwrap();
        } else if little + bigger < 2020 {
            index += 1;
            little = numbers[index];
        } else {
             break little * bigger;
        }
    }
}
