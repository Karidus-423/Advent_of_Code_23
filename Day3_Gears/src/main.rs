#![allow(non_snake_case)]

use reqwest;

fn main() {
    let document_url = "https://adventofcode.com/2023/day/3/input";

    let session_cookie = "_ga=GA1.2.400933693.1701306380; _ga_MHSNPJKWC7=GS1.2.1701439397.3.1.1701439518.0.0.0; session=53616c7465645f5f394579877731ffb5dc556c39284b49b784509ed6bbe8345ed048ec5b8d3a3a885d3c00b33f17e8119deec30625c144d031caaa2678a460bd; _gid=GA1.2.1699704916.1701436300";

    let response = reqwest::blocking::Client::new()
        .get(document_url)
        .header("cookie", format!("session={}", session_cookie))
        .send();
    if let Ok(document_response) = response {
        let calibration_doc = document_response.text();

        if let Ok(content) = calibration_doc {
            let input = content;
            run(&input)
        } else {
            eprintln!("Failed to read document content");
        }
    } else {
        eprintln!("Failed to fetch document");
    }
}
#[derive(Debug)]
struct PartNumber {
    number: i32,
    x1: i32,
    x2: i32,
    y: i32,
}

#[derive(Debug)]
struct Symbol {
    x: i32,
    y: i32,
    is_star: bool,
}

impl PartNumber {
    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        (self.x1 - 1..=self.x2 + 1).contains(&symbol.x)
            && (self.y - 1..=self.y + 1).contains(&symbol.y)
    }

    fn is_adjacent_to_any_symbol(&self, symbols: &[Symbol]) -> bool {
        symbols
            .iter()
            .any(|symbol| self.is_adjacent_to_symbol(symbol))
    }
}

impl Symbol {
    fn gear_ratio(&self, part_numbers: &[PartNumber]) -> i32 {
        if !self.is_star {
            return 0;
        }
        let mut ratio = 1;
        let mut number_of_adjacent_parts = 0;
        for pn in part_numbers {
            if pn.is_adjacent_to_symbol(self) {
                number_of_adjacent_parts += 1;
                if number_of_adjacent_parts > 2 {
                    return 0;
                }
                ratio *= pn.number;
            }
        }
        if number_of_adjacent_parts != 2 {
            return 0;
        }
        ratio
    }
}

fn parser(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut n: Option<i32> = None;
        let mut length: i32 = 0;
        for (x, &c) in line.as_bytes().iter().chain([b'.'].iter()).enumerate() {
            if c.is_ascii_digit() {
                length += 1;
                let digit = (c - b'0') as i32;
                n = n.map_or(Some(digit), |number| Some(number * 10 + digit));
            } else {
                if let Some(number) = n {
                    part_numbers.push(PartNumber {
                        number,
                        x1: x as i32 - length,
                        x2: x as i32 - 1,
                        y: y as i32,
                    });
                }
                n = None;
                length = 0;
                if c != b'.' {
                    symbols.push(Symbol {
                        x: x as i32,
                        y: y as i32,
                        is_star: c == b'*',
                    })
                }
            }
        }
    }
    (part_numbers, symbols)
}

pub fn run(input: &str) {
    let (part_numbers, symbols) = parser(input);

    let sum_of_valid_part_numbers: i32 = part_numbers
        .iter()
        .filter(|pn| pn.is_adjacent_to_any_symbol(&symbols))
        .map(|pn| pn.number)
        .sum();
    println!("{}", sum_of_valid_part_numbers);

    let sum_of_gear_ratios: i32 = symbols
        .iter()
        .map(|symbol| symbol.gear_ratio(&part_numbers))
        .sum();
    println!("{}", sum_of_gear_ratios);
}
