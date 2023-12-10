use std::{path::Path, io::{self, BufRead}, fs::File};

fn main() {
    if let Ok(lines) = read_lines("./input/day_1.txt") {
        let mut sum: i64 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("line: {ip}");

                let digitted_line = spelled_to_digits(ip.clone());
                println!("digitted line: {digitted_line}");

                let v = calibration_value(digitted_line);
                println!("calibration_value: {v}\n");

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

fn calibration_value(line: String) -> i64 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    for char in line.chars() {
        if !char.is_digit(10) {
            continue
        }

        if first_digit.is_none() {
            first_digit = Some(char);
            continue;
        }

        last_digit = Some(char);
    }

    if first_digit.is_some() && last_digit.is_none() {
        return format!("{}{}", first_digit.unwrap(), first_digit.unwrap()).parse().unwrap()
    }

    format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse().unwrap()
}

fn spelled_to_digits(mut line: String) -> String {
    let mut result = String::new();

    while line.len() != 0 {
        if line.starts_with("one") {
            result.push_str("1");
        } else if line.starts_with("two") {
            result.push_str("2");
        } else if line.starts_with("three") {
            result.push_str("3");
        } else if line.starts_with("four") {
            result.push_str("4");
        } else if line.starts_with("five") {
            result.push_str("5");
        } else if line.starts_with("six") {
            result.push_str("6");
        } else if line.starts_with("seven") {
            result.push_str("7");
        } else if line.starts_with("eight") {
            result.push_str("8");
        } else if line.starts_with("nine") {
            result.push_str("9");
        } else {
            result.push(line.chars().nth(0).unwrap())
        }

        line.drain(..1);
    }

    result
}
