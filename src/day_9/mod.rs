// On ne peut pas faire de recursion car trop longue
// Travailler sur un Vec de char plutôt et sur toute sa longueur
// S'inspirer de https://github.com/javorszky/adventofcode2024/blob/main/day09/src/part1.rs

use std::fs;
use std::io::Error;
use itertools::Itertools;

static PATH: &str = "src/day_9/data.txt";

pub async fn day_9() -> Result<u64, Error> {
    let string = fs::read_to_string(PATH)?;
    run_a(string.clone());
    Ok(run_b(string))
}

fn run_b(string: String) -> u64 {
    let _memory_representation_by_length = create_memory_representation_by_offset_and_length(&string);
    let optimized_free_space_at_the_end_file_in_block = optimize_free_space_at_the_end_file_in_block(_memory_representation_by_length);
    let memory_representation = create_memory_representation_by_mem_rep_by_length(optimized_free_space_at_the_end_file_in_block);
    let result = calculate_checksum(memory_representation);
    println!("Le resultat du b est {}", result);
    result
}

fn create_memory_representation_by_mem_rep_by_length(memory_by_length: Vec<(u32, String)>) -> Vec<String> {
    let mut result = vec![];
    
    for (number, string) in memory_by_length {
        for _index in 0..number {
            result.push(string.clone());
        }
    }
    
    result
}

fn optimize_free_space_at_the_end_file_in_block(memory: Vec<(u32, String)>) -> Vec<(u32, String)> {
    let mut optimized_memory = memory.clone();
    let mut second_optimized_memory = memory.clone();
    
    for mut index in 0..memory.len() {
        index = memory.len() - 1 - index;
        if optimized_memory[index].1 != "." {
            if let Some((position, point_block)) = second_optimized_memory.iter().find_position(|x| x.0 >= optimized_memory.get(index).unwrap().0 && x.1 == ".") {
                if position < index { 
                    
                    let point_block_length = point_block.0;
                    let current_block_length: u32 = optimized_memory.clone().get(index).unwrap().0;
                    
                    if point_block.0 == current_block_length {
                        optimized_memory.swap(position, index);
                    }
                    
                    if point_block_length > current_block_length { 
                        optimized_memory.remove(position);
                        optimized_memory.insert(position, (point_block_length - current_block_length, ".".to_string()));
                        optimized_memory.insert(position, (current_block_length, ".".to_string()));
                        optimized_memory.swap(position, index + 1);
                    }
                    
                    second_optimized_memory = optimized_memory.clone();
                }
            }
        }
    }
    
    optimized_memory
}

fn create_memory_representation_by_offset_and_length(string: &String) -> Vec<(u32, String)> {
    let mut result: Vec<(u32, String)>= vec![];
    
    for (index, number) in string.chars().enumerate() {
        let length = number.to_digit(10).unwrap_or(0);
        if index %  2 == 0 { 
            result.push((length, (index/2).to_string()));
            
        } else {
            result.push((length, ".".to_string()));
            
        }
    }
    
    result
}

fn run_a(string: String) -> u64 {
    let disk_map_representation = string;
    let memory_representation: Vec<String> = create_memory_representation(&disk_map_representation);
    let optimized_free_space_at_the_end_without_recursion = optimize_free_space_at_the_end_without_recursion(memory_representation); 
    let checksum = calculate_checksum(optimized_free_space_at_the_end_without_recursion);
    checksum
}

fn optimize_free_space_at_the_end_without_recursion(mut memory: Vec<String>) -> Vec<String> {
    for index in 0..memory.len() {
        if let Some(id) = memory.get(index) { 
            if !id.eq(".") {
                continue
            } 
            
            else {
                if let Some(last_number_index_reversed) = memory.iter().rev().position(|ch| !ch.eq(".")) {
                    
                    let last_number_index = memory.len() - last_number_index_reversed - 1;
                    if index > last_number_index { 
                        continue                        
                    }
                    memory.swap(index, last_number_index)
                }
            }
        }
    }
    
    memory
}

fn calculate_checksum(memory_representation: Vec<String>) -> u64 {
    let mut sum = 0;
    
    for (index, char) in memory_representation.iter().enumerate() {
        let value: u64 = char.parse().unwrap_or(0) as u64;
        sum = sum + (value * index as u64);
    }
    
    sum
}

fn create_memory_representation(disk_map_representation: &String) -> Vec<String> {
    let mut memory_representation: Vec<String> = vec![];
    
    for (index, char) in disk_map_representation.chars().enumerate() {
        let space_number_needed: u32 = char.to_digit(10).unwrap();
        
        for _number in  0..space_number_needed {
           if index % 2 == 0 {
               let id = index / 2;
               memory_representation.push(format!("{}", id))
           }

            if index % 2 == 1 {
               memory_representation.push(".".to_string())
    
            }
        }
    }
    
    memory_representation
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::vec;
    use super::*;

    // RUN_A
    #[test]
    fn test_test_data() {
        let test_string = fs::read_to_string("src/day_9/test_data.txt").unwrap();
        assert_eq!(run_a(test_string), 1928);
    }
    
    #[test]
    fn test_data() {
        let test_string = fs::read_to_string("src/day_9/data.txt").unwrap();
        assert_eq!(run_a(test_string), 6401092019345);
    }

    #[test]
    fn test_12345() {
        let test_string = String::from_str("12345").unwrap();
        assert_eq!(run_a(test_string), 60);
    }
    
    #[test]
    fn test_with_more_than_20_number() {
        let test_string = String::from_str("123232412123323422142").unwrap();
        assert_eq!(run_a(test_string), 1606);
    }

    // RUN_B
    #[test]
    fn test_test_data_b() {
        let test_string = fs::read_to_string("src/day_9/test_data.txt").unwrap();
        assert_eq!(run_b(test_string), 2858);
    }

    #[test]
    fn test_data_b() {
        let test_string = fs::read_to_string("src/day_9/data.txt").unwrap();
        assert_eq!(run_b(test_string), 6431472344710);
    }

    #[test]
    fn test_54321() {
        let test_string = String::from_str("54321").unwrap();
        assert_eq!(run_b(test_string), 31);
    }

    // CREATE_MEMORY_REPRESENTATION
    #[test]
    fn test_create_memory_representation() {
        let disk_map_representation = String::from_str("12345").unwrap();
        assert_eq!(create_memory_representation(&disk_map_representation),["0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2"]);
    }

    // OPTIMIZE MEMORY SPACE AT THE END WITHOUT RECURSION
    #[test]
    fn test_optimize_memory_space_at_the_end_1_without_recursion() {
        let disk_map_representation: Vec<String> = vec![
            String::from_str("0").unwrap(),
            String::from_str(".").unwrap(),
            String::from_str("1").unwrap()
        ];
        assert_eq!(optimize_free_space_at_the_end_without_recursion(disk_map_representation), vec!["0","1","."]);
    }
    
    #[test]
    fn test_optimize_memory_space_at_the_end_2_without_recursion() {
        let disk_map_representation = vec![String::from_str("0").unwrap(),
                                           String::from_str(".").unwrap(),
                                           String::from_str(".").unwrap(),
                                           String::from_str("1").unwrap(),
                                           String::from_str("1").unwrap(),
                                           String::from_str("1").unwrap(),
                                           String::from_str(".").unwrap(),
                                           String::from_str("2").unwrap()];
        assert_eq!(optimize_free_space_at_the_end_without_recursion(disk_map_representation), vec!["0","2","1","1","1",".",".","."]);
    }
    
    #[test]
    fn test_optimize_memory_space_without_recursion() {
        let disk_map_representation: Vec<String> = vec![String::from_str("0").unwrap(),
                                                        String::from_str(".").unwrap(),
                                                        String::from_str(".").unwrap(),
                                                        String::from_str("1").unwrap(),
                                                        String::from_str("1").unwrap(),
                                                        String::from_str("1").unwrap(),
                                                        String::from_str(".").unwrap(),
                                                        String::from_str(".").unwrap(),
                                                        String::from_str(".").unwrap(),
                                                        String::from_str(".").unwrap(),
                                                        String::from_str("2").unwrap(),
                                                        String::from_str("2").unwrap(),
                                                        String::from_str("2").unwrap(),
                                                        String::from_str("2").unwrap(),
                                                        String::from_str("2").unwrap()];
        assert_eq!(optimize_free_space_at_the_end_without_recursion(disk_map_representation), 
                   vec!["0","2","2","1","1","1","2","2","2",".",".",".",".",".","."]);
    }

    // CREATE MEMORY REPRESENTATION BY OFFSET AND LENGTH
    #[test]
    fn test_create_memory_representation_by_offset_and_length() {
        let disk_map_representation = String::from_str("12345").unwrap();
        assert_eq!(create_memory_representation_by_offset_and_length(&disk_map_representation),
                   [(1, String::from_str("0").unwrap()), 
                    (2, String::from_str(".").unwrap()), 
                    (3, String::from_str("1").unwrap()), 
                    (4, String::from_str(".").unwrap()), 
                    (5, String::from_str("2").unwrap())]);
    }
    
    // OPTIMIZE FREE SPACE AT THE END FILE IN BLOCK
    #[test]
    fn test_create_memory_representation_by_offset_and_length_0() {
        // 54321
        // 00000....111..2
        // 000002111......
        let disk_map_representation: Vec<(u32, String)> = vec![(5, String::from_str("0").unwrap()),
            (4, String::from_str(".").unwrap()),
            (3, String::from_str("1").unwrap()),
            (2, String::from_str(".").unwrap()),
            (1, String::from_str("2").unwrap())];
        assert_eq!(optimize_free_space_at_the_end_file_in_block(disk_map_representation),
                   [(5, String::from_str("0").unwrap()),
                       (1, String::from_str("2").unwrap()),
                       (3, String::from_str("1").unwrap()),
                       (3, String::from_str(".").unwrap()),
                       (2, String::from_str(".").unwrap()),
                       (1, String::from_str(".").unwrap())]);
    }

    // transform_mem_rep
    #[test]
    fn test_transform_mem_rep() {
        let disk_map_representation: Vec<(u32, String)> = vec![(5, String::from_str("0").unwrap()),
                                                               (1, String::from_str("2").unwrap()),
                                                               (3, String::from_str("1").unwrap()),
                                                               (3, String::from_str(".").unwrap()),
                                                               (2, String::from_str(".").unwrap()),
                                                               (1, String::from_str(".").unwrap())];
        assert_eq!(create_memory_representation_by_mem_rep_by_length(disk_map_representation),
                   vec![
                       String::from_str("0").unwrap(), // 0
                       String::from_str("0").unwrap(),
                       String::from_str("0").unwrap(),
                       String::from_str("0").unwrap(),
                       String::from_str("0").unwrap(),
                       String::from_str("2").unwrap(), // 5
                       String::from_str("1").unwrap(), // 6
                       String::from_str("1").unwrap(), // 7
                       String::from_str("1").unwrap(), // 8
                       String::from_str(".").unwrap(),
                       String::from_str(".").unwrap(),
                       String::from_str(".").unwrap(),
                       String::from_str(".").unwrap(),
                       String::from_str(".").unwrap(),
                       String::from_str(".").unwrap(),
                   ]);
    }
}
