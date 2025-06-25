use std::collections::HashMap;
use std::fs;
use std::io::Error;

static PATH: &str = "src/day_7/data.txt";

pub async fn day_8() -> Result<usize, Error> {
    let string = fs::read_to_string(PATH)?;
    run_a(&string);
    Ok(run_b(&string))
}

fn run_a(map_string: &String) -> usize {
    let map_length = map_string.lines().collect::<Vec<&str>>().len();

    let antenas_hash_map = build_antenas_hashmap(map_string);
    let mut antinodes_positions: HashMap<(usize, usize), char> = HashMap::new();

    for (current_position, antena) in antenas_hash_map.iter() {
        let other_same_antenas: HashMap<(usize, usize), char> = find_other_same_antenas(&antenas_hash_map, &current_position, antena);

        for (other_antena_position, _other_antena) in other_same_antenas.iter() {
            let potential_antinode = calculate_antinode_position(current_position, other_antena_position);
            if potential_antinode_is_in_the_map(map_length, &potential_antinode) {
                let antinode_position: (usize, usize) = (potential_antinode.0 as usize, potential_antinode.1 as usize);
                antinodes_positions.insert(antinode_position, '#');
            }
        }
    }

    println!("{:?}", antinodes_positions);
    antinodes_positions.len()
}

fn run_b(map_string: &String) -> usize {
    let map_length = map_string.lines().collect::<Vec<&str>>().len();

    let antenas_hash_map = build_antenas_hashmap(map_string);
    let mut antinodes_positions: HashMap<(usize, usize), char> = HashMap::new();

    for (current_position, antena) in antenas_hash_map.iter() {
        let other_same_antenas: HashMap<(usize, usize), char> = find_other_same_antenas(&antenas_hash_map, &current_position, antena);

        for (other_antena_position, _other_antena) in other_same_antenas.iter() {
            calculate_harmonic_antinode(map_length, &antenas_hash_map, &mut antinodes_positions, current_position, other_antena_position);
        }
    }
    
    for (antena_positon, antena)in antenas_hash_map.iter() {
        antinodes_positions.insert(*antena_positon, *antena);
    }

    println!("{:?}", antinodes_positions);
    antinodes_positions.len()
}

fn calculate_harmonic_antinode(map_length: usize, antenas_hash_map: &HashMap<(usize, usize), char>, antinodes_positions: &mut HashMap<(usize, usize), char>, position_1: &(usize, usize), position_2: &(usize, usize)) {
    let potential_harmonic_antinode = calculate_antinode_position(position_1, &position_2);
    if potential_antinode_is_in_the_map(map_length, &potential_harmonic_antinode) {
        let harmonic_antinode_position = (potential_harmonic_antinode.0 as usize, potential_harmonic_antinode.1 as usize);

        if potential_antinode_is_in_the_map(map_length, &potential_harmonic_antinode) {
            antinodes_positions.insert(harmonic_antinode_position, '#');
            calculate_harmonic_antinode(map_length, antenas_hash_map, antinodes_positions, position_2, &harmonic_antinode_position)
        }
    }
}

fn potential_antinode_is_in_the_map(map_length: usize, potential_antinode_position: &(isize, isize)) -> bool {
    potential_antinode_position.0 >= 0 && potential_antinode_position.0 < map_length as isize
        && potential_antinode_position.1 >= 0 && potential_antinode_position.1 < map_length as isize
}

fn build_antenas_hashmap(map_string: &String) -> HashMap<(usize, usize), char> {
    let mut hash_map: HashMap<(usize, usize), char> = HashMap::new();
    for (line_index, line) in map_string.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char != '.' {
                hash_map.insert((char_index, line_index), char);
            }
        }
    }
    hash_map
}

fn calculate_antinode_position(current_position: &(usize, usize), other_antena_position: &(usize, usize)) -> (isize, isize) {
    let antinode_x_positon: isize = (2 * other_antena_position.0) as isize - current_position.0 as isize;
    let antinode_y_position: isize = (2 * other_antena_position.1) as isize - current_position.1 as isize;
    (antinode_x_positon, antinode_y_position)
}

fn find_other_same_antenas(hash_map: &HashMap<(usize, usize), char>, current_position: &(usize, usize), antena: &char) -> HashMap<(usize, usize), char> {
    hash_map
        .clone().into_iter()
        .filter(
            |(key, other_antena)|
                other_antena == antena && key != current_position
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_antenas_produce_0_antinodes() {
        let input = ".....\n.....\n.....\n.....\n.....".to_string();

        assert_eq!(run_a(&input), 0);
    }

    #[test]
    fn test_3_antenas_produce_4_antinodes() {
        let input = "..a....\n.......\n..a....\n...a...\n.......\n.......\n.......".to_string();

        assert_eq!(run_a(&input), 4);
    }

    #[test]
    fn test_4_antenas_produce_12_antinodes() {
        let input = ".......\
                          \n..aa...\
                          \n..aa...\
                          \n.......\
                          \n.......".to_string();

        assert_eq!(run_a(&input), 12);
    }

    #[test]
    fn test_2_differents_antenas_produces_4_antinode() {
        let input = ".......\
                          \n..a....\
                          \n...a...\
                          \n.......\
                          \n.b.....\
                          \n...b...\
                          \n.......".to_string();
        assert_eq!(run_a(&input), 3);
    }

    #[test]
    fn test_2_differents_antenas_produces_superposed_antinodes() {
        let input = ".......\
                          \n..a....\
                          \n...a...\
                          \n.......\
                          \n.....b.\
                          \n......b\
                          \n.......".to_string();
        assert_eq!(run_a(&input), 2);
    }

    #[test]
    fn test_with_test_data() {
        let input = fs::read_to_string("src/day_8/test_data.txt").unwrap();
        assert_eq!(run_a(&input), 14);
    }

    #[test]
    fn test_with_real_data() {
        let input =  fs::read_to_string("src/day_8/data.txt").unwrap();
        assert_eq!(run_a(&input), 348);
    }

    // RUN_B
    #[test]
    fn test_3_differents_antenas_produces_11_antinode_with_harmonics() {
        let input = ".......\
                          \n..aa...\
                          \n....c..\
                          \n..b.c..\
                          \n...b...\
                          \n.......\
                          \n.......".to_string();
        assert_eq!(run_b(&input), 17);
    }

    #[test]
    fn test_1_antena_produces_5_antinodes_with_harmonics() {
        let input = "a......\
                          \n.a.....\
                          \n.......\
                          \n.......\
                          \n.......\
                          \n.......\
                          \n.......".to_string();
        assert_eq!(run_b(&input), 7);
    }

    #[test]
    fn test_b_with_test_data() {
        let input = fs::read_to_string("src/day_8/test_data.txt").unwrap();
        assert_eq!(run_b(&input), 34);
    }

    #[test]
    fn test_b_with_real_data() {
        let input = fs::read_to_string("src/day_8/data.txt").unwrap();
        assert_eq!(run_b(&input), 1221);
    }

    // FIND_OTHER_ANTENAS
    #[test]
    fn test_find_other_same_antenas() {
        let mut hash_map= HashMap::new();
        hash_map.insert((2,0), 'a');
        hash_map.insert((2,2), 'a');
        hash_map.insert((3,3), 'a');

        let mut result_hash_map = HashMap::new();
        result_hash_map.insert((2,2), 'a');
        result_hash_map.insert((3,3), 'a');

        assert_eq!(
            find_other_same_antenas(&hash_map, &(2, 0), &'a'),
            result_hash_map
        );
    }

    // CALCULATE_ANTINODE_POSITION
    #[test]
    fn test_calculate_antinodes_position() {
        assert_eq!(
            calculate_antinode_position(&(0, 0), &(1, 1)),
            (2, 2)
        );
    }

    #[test]
    fn test_calculate_antinodes_position_out_of_the_map() {
        assert_eq!(
            calculate_antinode_position(&(1, 1), &(0, 0)),
            (-1, -1)
        );
    }

    #[test]
    fn test_calculate_antinodes_position_out_of_the_map_2() {
        assert_eq!(
            calculate_antinode_position(&(3, 3), &(2, 0)),
            (1, -3)
        );
    }

    #[test]
    fn test_calculate_antinodes_position_out_of_the_map_3() {
        assert_eq!(
            calculate_antinode_position(&(2, 2), &(2, 0)),
            (2, -2)
        );
    }

}
