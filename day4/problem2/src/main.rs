use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let test = parse_file();
    let splitted_passport_data: Vec<&str> = test.split("\n\n").collect();

    let mut valid_passports: i32 = 0;

    for passport_data in splitted_passport_data.iter() {
        let splitted_fields: Vec<&str> = passport_data.split(|c| c == ' ' || c == '\n').collect();

        let mut fields_hashmap = HashMap::new();

        for field in splitted_fields {
            let splitted_fields = field.split(":").collect::<Vec<&str>>();
            fields_hashmap.insert(splitted_fields[0], splitted_fields[1]);
        }
        if check_data_for_fields(&mut fields_hashmap) {
            valid_passports += 1;
        }
    }

    println!("Valid passports: {}", valid_passports);
}

fn check_data_for_fields(fields_hashmap: &mut HashMap<&str, &str>) -> bool {
    //Remove the cid because we don't care about it in the validation
    fields_hashmap.remove(&"cid");

    let mut valid: i32 = 0;

    match fields_hashmap.get(&"byr") {
        Some(&value) => {
            let value_number = value.to_string().parse::<i32>().unwrap();
            if value_number >= 1920 && value_number <= 2002 {
                sum_valid(&mut valid);
            }
        }
        _ => {}
    };

    match fields_hashmap.get(&"iyr") {
        Some(&value) => {
            let value_number = value.to_string().parse::<i32>().unwrap();
            if value_number >= 2010 && value_number <= 2020 {
                sum_valid(&mut valid);
            }
        }
        _ => {}
    };

    match fields_hashmap.get(&"eyr") {
        Some(&value) => {
            let value_number = value.to_string().parse::<i32>().unwrap();
            if value_number >= 2020 && value_number <= 2030 {
                sum_valid(&mut valid);
            }
        }
        _ => {}
    };

    match fields_hashmap.get(&"hgt") {
        Some(&value) => {
            let re = Regex::new("[cm|in].").unwrap();
            if re.is_match(value) {
                let size_rule = re.find(value).unwrap().as_str();
                let regex_number = Regex::new("(\\d+)").unwrap();
                let number = regex_number
                    .find(value)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();

                if size_rule == "cm" && number >= 150 && number <= 193 {
                    sum_valid(&mut valid);
                } else if size_rule == "in" && number >= 59 && number <= 76 {
                    sum_valid(&mut valid);
                }
            }
        }
        _ => {}
    };

    match fields_hashmap.get(&"hcl") {
        Some(&value) => {
            let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            if re.is_match(value) {
                sum_valid(&mut valid);
            }
        }
        _ => {}
    };

    match fields_hashmap.get(&"ecl") {
        Some(&value) => {
            let valid_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

            if valid_values.contains(&value) {
                sum_valid(&mut valid);
            }
        }
        _ => {}
    };

    match fields_hashmap.get(&"pid") {
        Some(&value) => {
            let re = Regex::new(r"^[0-9]{9}$").unwrap();
            if re.is_match(value) {
                sum_valid(&mut valid);
            }
        }
        _ => {}
    };

    if valid == 7 {
        return true;
    }
    return false;
}

fn sum_valid(valid: &mut i32) {
    *valid += 1;
}

fn parse_file() -> String {
    let file = File::open("input.txt").expect("Can not open the file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}
