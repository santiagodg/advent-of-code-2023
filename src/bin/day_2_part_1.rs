use std::{path::Path, io::{self, BufRead}, fs::File};

use nom::{IResult, bytes::complete::{tag, take_while1}, number::complete::u16, character::{is_digit, is_space}, multi::{many1, separated_list1}, sequence::{Tuple, tuple}, branch::alt};

fn main() {
    if let Ok(lines) = read_lines("./input/day_2.txt") {
        let mut sum: u64 = 0;

        for line in lines {

            let g = game(&line.unwrap()).unwrap().1;

            let mut possible = true;
            for subset_of_cubes in g.subsets_of_cubes {
                if !subset_of_cubes.is_possible(12, 13, 14) {
                    possible = false;
                    break;
                }
            }

            if possible {
                sum += u64::from(g.id)
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

fn game(s: &str) -> IResult<&str, Game> {
    let is_digit = |c: char| c.is_digit(10);

    let (s, (_, id, _, subsets_of_cubes)) = (tag("Game "), take_while1(is_digit), tag(": "), separated_list1(tag("; "), subset_of_cubes)).parse(s)?;

    return Ok((s, Game{id: id.parse().unwrap(), subsets_of_cubes}))
}

fn subset_of_cubes(s: &str) -> IResult<&str, SubsetOfCubes> {
    let is_digit = |c: char| c.is_digit(10);

    let (s, subsets_of_cubes) = separated_list1(tag(", "), tuple((take_while1(is_digit), tag(" "), alt((tag("red"), tag("green"), tag("blue"))))))(s)?;

    let mut red: u8 = 0;
    let mut green: u8 = 0;
    let mut blue: u8 = 0;
    for (num, _, color) in subsets_of_cubes {
        match color {
            "red" => { red = num.parse().unwrap() },
            "green" => { green = num.parse().unwrap() },
            "blue" => { blue = num.parse().unwrap() },
            _ => panic!("unexpected color: {}", color),
        }
    }

    return Ok((s, SubsetOfCubes{ red: red, green: green, blue: blue }))
}

struct Game {
    pub id: u8,
    pub subsets_of_cubes: Vec<SubsetOfCubes>,
}

struct SubsetOfCubes {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl SubsetOfCubes {
    fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}
