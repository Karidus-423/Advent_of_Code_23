#![allow(non_snake_case)]
use itertools::Itertools;
use reqwest;

fn main() {
    let document_url = "https://adventofcode.com/2023/day/2/input";

    let session_cookie = "_ga=GA1.2.400933693.1701306380; _ga_MHSNPJKWC7=GS1.2.1701439397.3.1.1701439518.0.0.0; session=53616c7465645f5f394579877731ffb5dc556c39284b49b784509ed6bbe8345ed048ec5b8d3a3a885d3c00b33f17e8119deec30625c144d031caaa2678a460bd; _gid=GA1.2.1699704916.1701436300";

    let response = reqwest::blocking::Client::new()
        .get(document_url)
        .header("cookie", format!("session={}", session_cookie))
        .send();
    if let Ok(document_response) = response {
        let calibration_doc = document_response.text();

        if let Ok(content) = calibration_doc {
            // Split the content into lines
            let line: Vec<String> = content.lines().map(String::from).collect();
            parser(&line[0]);
            run(&content);
        } else {
            eprintln!("Failed to read document content");
        }
    } else {
        eprintln!("Failed to fetch document");
    }
}

fn parser(line: &str) -> (u32, [u32; 3]) {
    let mut parts = line.split(':');
    let header = parts.next().expect("Invalid line format (header)");
    let game_id: u32 = header["Game ".len()..]
        .parse()
        .expect("Invalid line format(game id)");

    let sets = parts.next().expect("Invalid line format (sets)");
    let mut maxs: [u32; 3] = [0, 0, 0];
    for n_color in sets.split(|c| c == ';' || c == ',') {
        let mut set_parts = n_color[1..].split_whitespace();
        let number: u32 = set_parts
            .next()
            .expect("Invalid line format (no number )")
            .parse()
            .expect("Invalid line format(number format)");
        let color = set_parts.next().expect("Invalid line format (no color)");
        let index = match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => unreachable!(),
        };
        maxs[index] = maxs[index].max(number);
    }
    (game_id, maxs)
}

fn is_possible(maxs: &[u32; 3]) -> bool {
    maxs[0] <= 12 && maxs[1] <= 13 && maxs[2] <= 14
}

fn power(maxs: &[u32; 3]) -> u32 {
    maxs[0] * maxs[1] * maxs[2]
}

pub fn run(input: &str) {
    let all_games = input.lines().map(parser).collect_vec();

    let sum_of_possible_game_ids: u32 = all_games
        .iter()
        .filter(|(_, maxs)| is_possible(maxs))
        .map(|(game_id, _)| game_id)
        .sum();
    println!("{}", sum_of_possible_game_ids);

    let sum_of_game_powers: u32 = all_games.iter().map(|(_, maxs)| power(maxs)).sum();
    println!("{}", sum_of_game_powers);
}
