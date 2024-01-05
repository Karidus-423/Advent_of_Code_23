#![allow(non_snake_case)]
use std::i32;

use reqwest;

fn main() {
    let document_url = "https://adventofcode.com/2023/day/1/input";

    let session_cookie = "_ga=GA1.2.400933693.1701306380; _ga_MHSNPJKWC7=GS1.2.1701439397.3.1.1701439518.0.0.0; session=53616c7465645f5f394579877731ffb5dc556c39284b49b784509ed6bbe8345ed048ec5b8d3a3a885d3c00b33f17e8119deec30625c144d031caaa2678a460bd; _gid=GA1.2.1699704916.1701436300";

    let response = reqwest::blocking::Client::new()
        .get(document_url)
        .header("cookie", format!("session={}", session_cookie))
        .send();
    if let Ok(document_response) = response {
        let calibration_doc = document_response.text();

        if let Ok(content) = calibration_doc {
            // Split the content into lines
            let lines: Vec<String> = content.lines().map(String::from).collect();
            part_1(&lines);
            part_2(&lines);
        } else {
            eprintln!("Failed to read document content");
        }
    } else {
        eprintln!("Failed to fetch document");
    }
}

fn part_1(lines: &Vec<String>) -> u32 {
    let mut sum = 0;

    for line in lines {
        let first_digit_left = line.chars().find(|c| c.is_digit(10));

        let first_digit_right = line.chars().rev().find(|c| c.is_digit(10));

        if let (Some(left), Some(right)) = (first_digit_left, first_digit_right) {
            let number = (left.to_digit(10).unwrap() * 10) + right.to_digit(10).unwrap();
            sum += number
        }
    }
    println!("Sum of all coordinates: {}", sum);
    sum
}

fn part_2(lines: &Vec<String>) {
    let find = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero", "1", "2",
        "3", "4", "5", "6", "7", "8", "9", "0",
    ];

    let mut finale: i32 = 0;

    for (_index, line) in lines.iter().enumerate() {
        let mut index = 0;
        let mut found_numbers = Vec::new();
        let mut clean_number: Vec<i32> = Vec::new();
        let mut found_cordinate: i32 = 0;

        while index < line.len() {
            let mut found = false;

            for &substring in &find {
                if line[index..].starts_with(substring) {
                    let start = index;
                    let _end = index + substring.len();

                    println!("OG: {}", line);

                    if let Some(number) = word_to_number(substring) {
                        println!("Converted to number: {}", number);

                        let i32_number: i32 = number.try_into().unwrap();
                        found_numbers.push(number);
                        clean_number.push(i32_number);
                    }
                    index = start + 1;
                    found = true;
                    break;
                }
            }

            if !found {
                index += 1;
            }
        }

        println!("Filtered Coordinate: {:?}", clean_number);
        match (clean_number.first(), clean_number.last()) {
            (Some(first), Some(last)) => {
                found_cordinate = format!("{}{}", first, last).parse().unwrap();
                println!("Found Coordinate: {}", found_cordinate);
            }
            _ => {
                println!("Coordinate not found");
            }
        }
        finale += found_cordinate;
        println!("----------------------------------");
        println!("Secret_coordinate: {}", finale);
    }
}

fn word_to_number(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "zero" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "0" => Some(0),
        _ => None,
    }
}
