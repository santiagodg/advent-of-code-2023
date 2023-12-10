use std::{path::Path, io::{self, BufRead}, fs::File};

fn main() {
    if let Ok(lines) = read_lines("./input/day_1.txt") {
        let mut sum = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let v = calibration_value(ip);
                sum = sum + v;
            }
        }

        println!("{sum}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calibration_value(line: String) -> i32 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    for char in line.chars() {
        if !char.is_digit(10) {
            continue
        }

        if first_digit.is_none() {
            first_digit = Some(char);
        }

        last_digit = Some(char);
    }

    format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse().unwrap()
}
