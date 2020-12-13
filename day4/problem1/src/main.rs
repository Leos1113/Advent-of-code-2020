use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let test = parse_file();
    let splitted_passport_data: Vec<&str> = test.split("\n\n").collect();

    let mut valid_data = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_passports: usize = 0;

    for passport_data in splitted_passport_data.iter() {
        let splitted_fields: Vec<&str> = passport_data.split(|c| c == ' ' || c == '\n').collect();

        let mut fields_vector = Vec::new();

        for field in splitted_fields {
            let field_name = field.split(":").collect::<Vec<&str>>()[0];
            fields_vector.push(field_name);
        }

        fields_vector.retain(|idx| !idx.is_empty());
        fields_vector.retain(|idx| idx != &"cid");

        fields_vector.sort();
        valid_data.sort();

        let matching = valid_data
            .iter()
            .zip(&fields_vector)
            .filter(|&(a, b)| a == b)
            .count();

        if matching == valid_data.len() {
            valid_passports += 1;
        }
    }

    println!("Valid passports: {}", valid_passports);
}

fn parse_file() -> String {
    let file = File::open("input.txt").expect("Can not open the file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}
