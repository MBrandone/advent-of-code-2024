use std::collections::HashMap;
use std::fs;
use std::io::Error;

static PATH: &str = "src/day_10/data.txt";

pub async fn day_10() -> Result<u32, Error> {
    let string = fs::read_to_string(PATH)?;
    Ok(run_a(string))
}

fn run_a(input: String) -> u32 {
    let map_length = input.lines().collect::<Vec<&str>>().len();
    let hashmap = create_hashmap_map(&input);
    
    let mut found_paths: Vec<Vec<(usize, usize)>> = vec![];

    for (index_y, line) in input.lines().enumerate() {
        for (index_x, char) in line.chars().enumerate() {
            
            if let Some(_number_0) = is_it_a_0(char) {
                let mut current_path: Vec<(usize, usize)> = vec![];
                let position_0 = (index_x, index_y);
                current_path.push(position_0);
                
                find_path(map_length, &hashmap, &mut found_paths, &mut current_path, position_0, 1);
                
            }    
        }
    }
    
    let score_hashmap = calculate_score_for_trailhead(&found_paths);
    
    println!("Le hashmap score : {:?}", score_hashmap.keys().len());
    println!("Le hashmap score rating : {:?}", score_hashmap.values().sum::<u32>());
    // B : 1925
    
    score_hashmap.keys().len() as u32
}

fn calculate_score_for_trailhead(found_paths: &Vec<Vec<(usize, usize)>>) -> HashMap<(usize, usize, usize, usize), u32> {
    let mut results: HashMap<(usize, usize, usize, usize), u32> = HashMap::new();

    for found_path in found_paths {
        let trail_head = found_path.get(0).unwrap();
        let final_position = found_path.get(9).unwrap();
        
        let beginning_end_position = (trail_head.0, trail_head.1, final_position.0, final_position.1);
        
        if results.contains_key(&beginning_end_position) { 
            results.insert(beginning_end_position.clone(), results.get(&beginning_end_position).unwrap() + 1);
        } else { 
            results.insert(beginning_end_position.clone(), 1);
        }
        
    }
    
    results
}

const FINAL_NUMBER: u32 = 9;

fn find_path(map_length: usize, hashmap: &HashMap<(usize, usize), u32>, found_paths: &mut Vec<Vec<(usize, usize)>>, current_path: &mut Vec<(usize, usize)>, current_position: (usize, usize), searched_number: u32) { 
    if let Some(number_positions) = find_number_in_neighbours(current_position, &hashmap, searched_number, map_length) {
        
        for position in number_positions {
            let mut path_for_this_position = current_path.clone();

            path_for_this_position.push(position);

            if searched_number == FINAL_NUMBER { 
                found_paths.push(path_for_this_position.clone());
                continue
            }
            
            find_path(map_length, hashmap, found_paths, &mut path_for_this_position, position, searched_number + 1)
        }
    } 
}

fn find_number_in_neighbours(current_position: (usize, usize), map: &HashMap<(usize, usize), u32>, searched_number: u32, map_length: usize) -> Option<Vec<(usize, usize)>> {
    let relative_positions_to_inspect: [(isize, isize); 4] = [
        (-1, 0),
        (1,  0),
        (0, -1),
        (0,  1),
    ];
    
    let mut found_positions: Vec<(usize, usize)> = vec![];

    for relative_position_to_inspect in relative_positions_to_inspect {
        if let Some(position_to_inspect) = calculate_position_to_inspect(current_position, relative_position_to_inspect, map_length){ 
            if let Some(position_found) = map.get(&position_to_inspect) { 
                if *position_found == searched_number {
                    found_positions.push(position_to_inspect)
                }
            }
        
        }
    }
    
    if found_positions.len() != 0 {
        return Some(found_positions)
    }
    
    None
}

fn calculate_position_to_inspect(current_position: (usize, usize), relative_position_to_inspect: (isize, isize), map_length: usize) -> Option<(usize, usize)> {
    let x_to_inspect: isize = current_position.0 as isize + relative_position_to_inspect.0;
    let y_to_inspect: isize = current_position.1 as isize + relative_position_to_inspect.1;
    
    if x_to_inspect >= 0 && x_to_inspect < map_length as isize && y_to_inspect >= 0 && y_to_inspect < map_length as isize { 
        return Some((x_to_inspect as usize, y_to_inspect as usize))    
    }
    
    None
    
}

fn is_it_a_0(char: char) -> Option<u8> {
    if char.is_numeric() { 
        let number = char.to_digit(10).unwrap();
        if number == 0 { 
            return Some(0)
        }
    }
    
    None
}

fn create_hashmap_map(string: &String) -> HashMap<(usize, usize), u32> {
    let mut hashmap: HashMap<(usize, usize), u32> = HashMap::new();

    for (index_y, line) in string.lines().enumerate() {
        for (index_x, char) in line.chars().enumerate() {
            hashmap.insert((index_x, index_y), char.to_digit(10).unwrap());
        }
    }

    hashmap
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    // RUN_A
    #[test]
    fn test_test_data() {
        let test_string = fs::read_to_string("src/day_10/test_data.txt").unwrap();
        assert_eq!(run_a(test_string), 36);
    }

    #[test]
    fn test_data() {
        let test_string = fs::read_to_string("src/day_10/data.txt").unwrap();
        assert_eq!(run_a(test_string), 778);
    }

    #[test]
    fn test_minimal_data() {
        let test_string = "0123\n1234\n8765\n9876".to_string();
        assert_eq!(run_a(test_string), 1);
    }

    // IS IT A 0
    #[test]
    fn test_is_it_a_0_for_0() {
        assert_eq!(is_it_a_0('0'), Some(0));
    }

    #[test]
    fn test_is_it_a_0_for_1() {
        assert_eq!(is_it_a_0('1'), None);
    }
    
    // FIND NUMBER IN NEIGHBOURS
    #[test]
    fn test_is_there_a_1_false() {
        let test_string = "000\n000\n000\n".to_string();
        let hashmap = create_hashmap_map(&test_string);
        assert_eq!(find_number_in_neighbours((1, 1), &hashmap, 1, 3),
                   None);
    }

    #[test]
    fn test_is_there_a_1_true_with_1() {
        let test_string = "010\n000\n000\n".to_string();
        let hashmap = create_hashmap_map(&test_string);
        assert_eq!(
            find_number_in_neighbours((1, 1), &hashmap, 1, 3),
            Some(vec![(1, 0)])
        );
    }
}