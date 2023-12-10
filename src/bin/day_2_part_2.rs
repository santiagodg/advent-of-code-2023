use std::{path::Path, io::{self, BufRead}, fs::File, cmp::max};

use nom::{IResult, bytes::complete::{tag, take_while1}, multi::separated_list1, sequence::{Tuple, tuple}, branch::alt};

fn main() {
    if let Ok(lines) = read_lines("./input/day_2.txt") {
        let mut sum: u64 = 0;

        for line in lines {

            let g = game(&line.unwrap()).unwrap().1;

            let mut red: u8 = 0;
            let mut green: u8 = 0;
            let mut blue: u8 = 0;

            for subset_of_cubes in g.subsets_of_cubes {
                red = max(red, subset_of_cubes.red);
                green = max(green, subset_of_cubes.green);
                blue = max(blue, subset_of_cubes.blue);
            }

            sum += u64::from(red) * u64::from(green) * u64::from(blue);
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

    let (s, (_, _, _, subsets_of_cubes)) = (tag("Game "), take_while1(is_digit), tag(": "), separated_list1(tag("; "), subset_of_cubes)).parse(s)?;

    return Ok((s, Game{subsets_of_cubes}))
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
    pub subsets_of_cubes: Vec<SubsetOfCubes>,
}

struct SubsetOfCubes {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
