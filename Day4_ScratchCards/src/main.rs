#![allow(non_snake_case)]

use reqwest;

fn main() {
    let document_url = "https://adventofcode.com/2023/day/4/input";

    let session_cookie = "_ga=GA1.2.400933693.1701306380; _ga_MHSNPJKWC7=GS1.2.1701439397.3.1.1701439518.0.0.0; session=53616c7465645f5f394579877731ffb5dc556c39284b49b784509ed6bbe8345ed048ec5b8d3a3a885d3c00b33f17e8119deec30625c144d031caaa2678a460bd; _gid=GA1.2.1699704916.1701436300";

    let response = reqwest::blocking::Client::new()
        .get(document_url)
        .header("cookie", format!("session={}", session_cookie))
        .send();
    if let Ok(document_response) = response {
        let calibration_doc = document_response.text();

        if let Ok(content) = calibration_doc {
            let (points_sum, multipler_sum) = parser(&content);
            println!("Points sum: {}", points_sum);
            println!("Multipler sum: {}", multipler_sum);
        } else {
            eprintln!("Failed to read document content");
        }
    } else {
        eprintln!("Failed to fetch document");
    }
}

fn parser(data: &str) -> (usize, usize) {
    let mut split_idx: Option<usize> = None;
    let mut multipler = vec![1; data.lines().count()];
    let mut points = vec![0; data.lines().count()];
    for (idx, line) in data.lines().enumerate() {
        let mut card: Vec<&str> = line.split_once(':').unwrap().1.split_whitespace().collect();
        if split_idx.is_none() {
            split_idx = card.iter().position(|&x| x == "|");
        }
        card.remove(split_idx.unwrap());
        let mut winning: Vec<_> = card.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let mine = winning.split_off(split_idx.unwrap());
        let matches = mine
            .iter()
            .map(|x| winning.contains(x) as usize)
            .sum::<usize>(); // wins copy of next N
        for game in idx + 1..idx + 1 + matches {
            multipler[game] += multipler[idx];
        }
        points[idx] = if matches == 0 {
            0
        } else {
            2_usize.pow((matches - 1) as u32)
        };
    }
    (points.iter().sum(), multipler.iter().sum())
}
