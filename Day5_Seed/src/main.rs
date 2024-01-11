#![allow(non_snake_case)]
use reqwest;
use std::cmp::min;

fn main() {
    let document_url = "https://adventofcode.com/2023/day/5/input";

    let session_cookie = "_ga=GA1.2.400933693.1701306380; _ga_MHSNPJKWC7=GS1.2.1701439397.3.1.1701439518.0.0.0; session=53616c7465645f5f394579877731ffb5dc556c39284b49b784509ed6bbe8345ed048ec5b8d3a3a885d3c00b33f17e8119deec30625c144d031caaa2678a460bd; _gid=GA1.2.1699704916.1701436300";

    let response = reqwest::blocking::Client::new()
        .get(document_url)
        .header("cookie", format!("session={}", session_cookie))
        .send();
    if let Ok(document_response) = response {
        let calibration_doc = document_response.text();
        if let Ok(content) = calibration_doc {
            let sections = process_data(&content);
            let p1 = part1(&sections);
            let p2 = part2(&sections);
            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
        } else {
            eprintln!("Failed to read document content");
        }
    } else {
        eprintln!("Failed to fetch document");
    }
}

fn process_data(data: &str) -> Vec<Vec<usize>> {
    let ret = data
        .split("\n\n")
        .map(|x| {
            x.split_once(':')
                .unwrap()
                .1
                .replace('\n', " ")
                .split_whitespace()
                .map(|z| z.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    ret
}
fn part1(sections: &Vec<Vec<usize>>) -> usize {
    let mut target_vals = sections[0].clone().to_owned();
    for section in sections[1..].iter() {
        'target: for target in target_vals.iter_mut() {
            for chunk in section.chunks(3) {
                if let &[dest_start, source_start, map_range] = chunk {
                    // each time we use a vec of present values to build a map, initial vec is seeds
                    if (source_start <= *target) && ((source_start + map_range) >= *target) {
                        *target = dest_start + *target - source_start;
                        continue 'target;
                    }
                }
            } // if not matched target remains unchanged
        }
    }
    *target_vals.iter().min().unwrap()
}

fn part2(sections: &Vec<Vec<usize>>) -> usize {
    let mut next_target = sections[0].clone().to_owned();
    for section in sections[1..].iter() {
        let mut target_vals = next_target.clone();
        next_target.clear();

        'target: while let (Some(target_range), Some(target_start)) =
            (target_vals.pop(), target_vals.pop())
        {
            let target_end = target_start + target_range;

            for chunk in section.chunks(3) {
                if let &[dest_start, source_start, map_range] = chunk {
                    let source_end = source_start + map_range;

                    // check if target and source have overlap
                    if (target_start <= source_end) && (source_start <= target_end) {
                        // in case of partial match add the non-overlapped parts back to vec
                        if target_start < source_start {
                            target_vals.push(target_start);
                            target_vals.push(source_start - target_start - 1);
                        }

                        if target_end > source_end {
                            target_vals.push(source_end + 1);
                            target_vals.push(target_end - source_end - 1);
                        }

                        // get the overlap
                        let (offset, overlap_start) = if source_start < target_start {
                            (target_start - source_start, target_start)
                        } else {
                            (0, source_start)
                        };
                        let overlap_range = min(target_end, source_end) - overlap_start;
                        next_target.push(dest_start + offset);
                        next_target.push(overlap_range);
                        continue 'target;
                    }
                }
            }
            next_target.push(target_start);
            next_target.push(target_range);
        }
    }
    *next_target
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if (i % 2) == 0 { Some(v) } else { None })
        .min()
        .unwrap()
}
