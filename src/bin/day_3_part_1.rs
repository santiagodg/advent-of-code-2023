use std::{path::Path, io::{self, BufRead}, fs::File};

fn main() {
    if let Ok(lines) = read_lines("./input/day_3.txt") {
        let mut sum: u64 = 0;

        let mut symbol_registry: Vec<(u8, u8)> = vec![];
        let mut number_registry: Vec<(((u8, u8), (u8, u8)), u16)> = vec![];

        for (y, line) in lines.enumerate() {
            let line = line.unwrap();

            let mut begin_pos: (u8, u8) = (0, 0);
            let mut end_pos: (u8, u8);
            let mut num_buffer = String::new();
            let mut reading_number = false;


            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    println!("Found point: {:?}", (x, y));

                    if reading_number {
                        reading_number = false;
                        end_pos = (u8::try_from(x-1).unwrap(), u8::try_from(y).unwrap());

                        let number: u16 = num_buffer.parse().unwrap();

                        if number_is_adjacent_to_symbols((begin_pos, end_pos), &symbol_registry) {
                            println!("Adding number due to previously registered symbol: {}", number);
                            sum += u64::from(number);
                        } else {
                            println!("Registering number: {} at ({:?}, {:?})", number, begin_pos, end_pos);
                            number_registry.push(((begin_pos, end_pos), number));
                            println!("Number registry: {:?}", number_registry);
                        }

                        num_buffer = String::new();
                        begin_pos = (0, 0);
                    }

                    continue;
                }

                if c.is_digit(10) {
                    if !reading_number {
                        reading_number = true;
                        begin_pos = (u8::try_from(x).unwrap(), u8::try_from(y).unwrap());
                    }

                    num_buffer.push(c);

                    continue;
                }

                // found symbol
                println!("Recognized symbol: {} at {:?}", c, (x, y));
                println!("Registering symbol at {:?}", (u8::try_from(x).unwrap(), u8::try_from(y).unwrap()));
                symbol_registry.push((u8::try_from(x).unwrap(), u8::try_from(y).unwrap()));
                println!("Symbol registry: {:?}", symbol_registry);

                if reading_number {
                    println!("Found symbol while reading number");

                    reading_number = false;
                    end_pos = (u8::try_from(x-1).unwrap(), u8::try_from(y).unwrap());

                    let number: u16 = num_buffer.parse().unwrap();

                    println!("Finished reading number: {} at ({:?}, {:?})", number, begin_pos, end_pos);

                    if number_is_adjacent_to_symbols((begin_pos, end_pos), &symbol_registry) {
                        println!("Adding number due to previously registered symbol: {}", number);
                        sum += u64::from(number);
                    } else {
                        println!("Registering number: {} at ({:?}, {:?})", number, begin_pos, end_pos);
                        number_registry.push(((begin_pos, end_pos), number));
                        println!("Number registry: {:?}", number_registry);
                    }

                    num_buffer = String::new();
                    begin_pos = (0, 0);
                }

                println!("Number registry: {:?}", number_registry);

                let (number, ok) = symbol_is_adjacent_to_numbers((u8::try_from(x).unwrap(), u8::try_from(y).unwrap()), &mut number_registry);
                if ok {
                    println!("Adding number due to previously registered number: {}", number);
                    sum += number;
                }
            }

            if reading_number {
                end_pos = (u8::try_from(line.chars().count() - 1).unwrap(), u8::try_from(y).unwrap());

                let number: u16 = num_buffer.parse().unwrap();

                if number_is_adjacent_to_symbols((begin_pos, end_pos), &symbol_registry) {
                    println!("Adding number due to previously registered symbol: {}", number);
                    sum += u64::from(number);
                } else {
                    println!("Registering number: {} at ({:?}, {:?})", number, begin_pos, end_pos);
                    number_registry.push(((begin_pos, end_pos), number));
                    println!("Number registry: {:?}", number_registry);
                }
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

fn number_is_adjacent_to_symbols((begin_pos, end_pos): ((u8, u8), (u8, u8)), symbol_registry: &Vec<(u8, u8)>) -> bool {
    for symbol_pos in symbol_registry {
        if begin_pos.0 == 0 && begin_pos.1 == 0 {
            if begin_pos.0 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
                return true;
            }
        } else if begin_pos.0 == 0 {
            if begin_pos.0 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 - 1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
                return true;
            }
        } else if begin_pos.1 == 0 {
            if begin_pos.0 - 1 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
                return true;
            }
        } else if begin_pos.0 - 1 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 - 1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
            return true;
        }
    }

    return false
}

fn symbol_is_adjacent_to_numbers(symbol_pos: (u8, u8), number_registry: &mut Vec<(((u8, u8), (u8, u8)), u16)>) -> (u64, bool) {
    let mut found = false;
    let mut sum: u64 = 0;
    let mut numbers_to_delete: Vec<usize> = vec![];

    println!("Number registry: {:?}", number_registry);

    for (i, ((begin_pos, end_pos), number)) in number_registry.clone().into_iter().enumerate() {
        println!("Checking number: {} at ({:?}, {:?})", number, begin_pos, end_pos);

        if begin_pos.0 == 0 && begin_pos.1 == 0 {
            if begin_pos.0 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
                println!("Found adjacent number from symbol: {} at ({:?}, {:?})", number, begin_pos, end_pos);

                sum += u64::from(number);
                found = true;

                numbers_to_delete.push(i);
            }
        } else if begin_pos.0 == 0 {
            if begin_pos.0 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 - 1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
                println!("Found adjacent number from symbol: {} at ({:?}, {:?})", number, begin_pos, end_pos);

                sum += u64::from(number);
                found = true;

                numbers_to_delete.push(i);
            }
        } else if begin_pos.1 == 0 {
            if begin_pos.0 - 1 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
                println!("Found adjacent number from symbol: {} at ({:?}, {:?})", number, begin_pos, end_pos);

                sum += u64::from(number);
                found = true;

                numbers_to_delete.push(i);
            }
        } else if begin_pos.0 - 1 <= symbol_pos.0 && symbol_pos.0 <= end_pos.0 + 1 && begin_pos.1 - 1 <= symbol_pos.1 && symbol_pos.1 <= end_pos.1 + 1 {
            println!("Found adjacent number from symbol: {} at ({:?}, {:?})", number, begin_pos, end_pos);

            sum += u64::from(number);
            found = true;

            numbers_to_delete.push(i);
        }
    }

    println!("Indices to delete: {:?}", numbers_to_delete);

    for number_to_delete in numbers_to_delete.into_iter().rev() {
        number_registry.remove(number_to_delete);
    }

    println!("Number registry after trying to find if symbol is adjacent to number registry: {:?}", number_registry);

    (sum, found)
}
