use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let content = read_file("input.txt");

    let mut input_numbers = content_lines_to_integer(content);

    input_numbers.sort();

    println!("{}", multiply_the_three_numbers_that_sum_2020(input_numbers));
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

fn multiply_the_three_numbers_that_sum_2020(numbers: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..numbers.len() - 2 {
        for j in i+1..numbers.len() - 1 {
            for k in j+1..numbers.len() {
                if numbers[i] + numbers[j] +  numbers[k] == 2020 {
                    result =numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }

    return result;
}
