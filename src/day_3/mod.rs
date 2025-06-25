use std::fs;
use regex::Regex;

pub async fn day_3() {
    let input_string = fs::read_to_string("src/day_3/data.txt").unwrap();

    let regex_mul_string_with_numbers: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mul_matches: Vec<_> = find_matching_strings(&input_string, regex_mul_string_with_numbers);
    let mul_score = calculate_score(mul_matches);
    println!("🔍 score: {}", mul_score);

    let regex_mul_string_with_numbers_and_do_and_dont: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let matches_with_do_and_dont =
        find_matching_strings(&input_string, regex_mul_string_with_numbers_and_do_and_dont);
    let score_with_do_and_dont = calculate_matches_with_do_and_dont(matches_with_do_and_dont);
    println!("🔍 score with do and dont : {}", score_with_do_and_dont);
}

fn find_matching_strings(string: &String, regex: Regex) -> Vec<&str> {
    let mut matches: Vec<_> = Vec::new();
    for a_match in regex.find_iter(string) {
        matches.push(a_match.as_str());
    }
    matches
}

fn calculate_matches_with_do_and_dont(matches: Vec<&str>) -> i32 {
    let mut matches_without_dont: Vec<String> = Vec::new();
    let regex_do: Regex = Regex::new(r"do\(\)").unwrap();
    let regex_dont: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut dont_regex_is_on = false;
    for a_matche in matches.iter() {
        if regex_dont.is_match(a_matche) {
            dont_regex_is_on = true;
            continue;
        }
        if regex_do.is_match(a_matche) {
            dont_regex_is_on = false;
            continue;
        }
        if dont_regex_is_on {
            continue;
        }
        if !dont_regex_is_on {
            matches_without_dont.push(a_matche.parse().unwrap())
        }
    }

    calculate_score(matches)
}

fn calculate_score(matches: Vec<&str>) -> i32 {
    let mut score: i32 = 0;
    let regex_max_3_numbers = Regex::new(r"\d{1,3}").unwrap();

    for matche in matches {
        let mut product = 1;
        for number in regex_max_3_numbers.find_iter(&matche) {
            let real_number: i32 = number.as_str().parse().unwrap();
            product = product * real_number;
        }
        score = score + product;
    }
    score
}
