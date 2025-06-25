use itertools::Itertools;
use std::{fs, io};
use std::collections::HashMap;

const PATH: &'static str = "src/day_1/data.txt";

pub async fn day_1() -> io::Result<Option<(u32, usize)>> {
    let input_string = fs::read_to_string(PATH)?;
    let (first_list, second_list) = read_file_to_vector(input_string);

    let ordered_first_list: Vec<u32> = first_list.iter().copied().sorted().collect();
    let ordered_second_list: Vec<u32> = second_list.iter().copied().sorted().collect();

    let distances = calculate_distances(&ordered_first_list, &ordered_second_list);

    let distances_sum = distances.iter().sum();
    let similarity = calculate_similarity(&ordered_first_list, &ordered_second_list);

    println!("Distance result is {}", distances_sum);
    println!("Similarity result is {}", similarity);
    
    Ok(Some((distances_sum, similarity)))
}

fn read_file_to_vector(input_string: String) -> (Vec<u32>, Vec<u32>) {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    const SEPARATOR_ITEM_FIRST_AND_SECOND_LIST: &'static str = "   ";
    
    for line in input_string.lines() {
        if let Some(index) = line.find(SEPARATOR_ITEM_FIRST_AND_SECOND_LIST) {
            let (first_half, second_half) = line.split_at(index);
            first_list.push(parse_string_to_number(first_half));
            second_list.push(parse_string_to_number(second_half));
        }
    }

    (first_list, second_list)
}

fn parse_string_to_number(string: &str) -> u32 {
    string
        .trim()
        .parse()
        .expect("One of the value in the data set is not a number.")
}

fn calculate_distances(first_list: &[u32], second_list: &[u32]) -> Vec<u32> {
    first_list.iter()
        .zip(second_list)
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .collect()
}

fn calculate_similarity(first_list: &[u32], second_list: &[u32]) -> usize {
    let mut counts = HashMap::new();
    for &num in second_list {
        *counts.entry(num).or_insert(0) += 1;
    }
    first_list.iter()
        .map(|&num| counts.get(&num).unwrap_or(&0) * num as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distances(&vec![2, 3], &vec![4, 6]), vec![2, 3]);
    }

    #[test]
    fn test_calculate_similarity() {
        assert_eq!(
            calculate_similarity(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]),
            31
        );
    }
}
