use std::io::{BufReader, Read};
use std::fs::File;

struct Password {
    repetition_policy: String,
    letter_policy: String,
    password_value: String
}

fn main() {
    let content = read_file("input.txt");

    let mut count_valids = 0;

    for line in content.lines() {
        let splitted_data: Vec<&str>  = line.split(" ").collect();

        let password = Password{
            repetition_policy: splitted_data[0].to_string(),
            letter_policy: splitted_data[1].to_string(),
            password_value: splitted_data[2].to_string()
        };

        if password.is_valid() {
            count_valids += 1;
        }
    }

    println!("{} are valids!", count_valids);
}

impl Password {
    fn is_valid(&self) -> bool {
        let letter = self.letter_policy.replace(":", "").chars().last().unwrap();
        let count_policies: Vec<&str> = self.repetition_policy.split("-").collect();

        return xor(
            self.password_value.chars().nth(count_policies[0].to_string().parse::<usize>().unwrap() - 1) == Some(letter),
            self.password_value.chars().nth(count_policies[1].to_string().parse::<usize>().unwrap() - 1) == Some(letter));
    }
}

fn xor(a: bool, b: bool) -> bool {
    return (a || b) && !(a && b);
}

fn read_file(filename: &str) -> String {
    let file = File::open(filename).expect("Can not open the file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    return contents;
}
