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
            let mut sum = 0;

            for line in &lines {
                let first_digit_left = line.chars().find(|c| c.is_digit(10));
                
                let first_digit_right = line.chars().rev().find(|c| c.is_digit(10));

                if let(Some(left), Some(right)) = (first_digit_left,first_digit_right){
                    let number = (left.to_digit(10).unwrap()*10) + right.to_digit(10).unwrap();
                    sum += number
                }
            }
            println!("Sum of all coordinates: {}", sum)

        } else {
            eprintln!("Failed to read document content");
        }
    } else {
        eprintln!("Failed to fetch document");
    }
}
