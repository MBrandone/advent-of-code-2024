use std::fs;
use std::io::Error;
use crate::day_7::CalculOperator::{Add, Concat, Multiply};

static PATH: &str = "src/day_7/data.txt";

#[derive(Debug)]
#[derive(PartialEq)]
enum CalculOperator {
    Add,
    Multiply,
    Concat,
    Unknown,
    // Est-ce-que c'est facile d'ajouter un nouvel opérateur ?
    // => les match sur operator demande de définir l'opération
    // => ça ne demande pas mais ça devrait => Quand je crée des combinaisons avec + d'opérateur => Comment je transformer 3 en Soustract 
    //Soustract
}

pub async fn day_7() -> u64 {
    let result_a = run_a(PATH).unwrap();
    println!("Le résultat est {}", result_a);
    
    let result_b = run_b(PATH).unwrap();
    println!("Le résultat avec le concat operator est {}", result_b);
    
    result_b
}

fn run_a(path: &str) -> Result<u64, Error> {
    let lines_to_verify = transform_to_lines(path)?;
    run(lines_to_verify, vec![Add, Multiply])
}


fn run_b(path: &str) -> Result<u64, Error> {
    let lines_to_verify = transform_to_lines(path)?;
    run(lines_to_verify, vec![Add, Multiply, Concat])
}

fn transform_to_lines(path: &str) -> Result<Vec<(u64, Vec<u64>)>, Error> {
    let string = fs::read_to_string(path)?;
    Ok(parse_string_to_operations(string))
}

fn run(lines_to_verify: Vec<(u64, Vec<u64>)>, calcul_operators: Vec<CalculOperator>) -> Result<u64, Error> {
    let mut operations_with_results = vec![];

    for (line_index, line_to_verify) in lines_to_verify.iter().enumerate() {
        println!("Travail sur la ligne numero {}", line_index);
        let operation_to_verify = line_to_verify.clone();
        let expected_result = operation_to_verify.0;
        let operation_members = operation_to_verify.1;

        let operations_to_apply = create_operations_to_apply(&operation_members, calcul_operators.len() as u32);

        for (operator_serie_index, operator_series) in operations_to_apply.iter().enumerate() {
            let mut current_result = operation_members.get(0).unwrap_or(&0).clone();

            for (operator_index, operator) in operator_series.iter().enumerate() {
                let next_member = operation_members.get(operator_index + 1).unwrap().clone();
                current_result = operate(operator, current_result, next_member);
            }

            println!("Pour l'opération {}, le résultat est {}, alors que l'attendu est {}", operator_serie_index, current_result, expected_result);
            if current_result == expected_result {
                operations_with_results.push(current_result.clone());
                break
            }
        }

    }

    println!("{:?}", operations_with_results);

    Ok(operations_with_results.iter().sum())
}

fn operate(operator: &CalculOperator, member_1: u64, member_2: u64) -> u64 {
    match operator {
        Add => member_1 + member_2,
        Multiply => member_1 * member_2,
        Concat => {
            let member_1_string = member_1.to_string();
            let member_2_string = member_2.to_string();
            format!("{}{}", member_1_string, member_2_string).parse().unwrap()
        }
        CalculOperator::Unknown => member_1
    }
}

fn create_operations_to_apply(operators: &Vec<u64>, calcul_operator_number: u32) -> Vec<Vec<CalculOperator>> {
    let binaries_string = get_all_operators_combinations(operators, calcul_operator_number);
    transform_to_calcul_operators_combinations(binaries_string)
}

fn transform_to_calcul_operators_combinations(string_combinations: Vec<String>) -> Vec<Vec<CalculOperator>> {
    string_combinations
        .iter()
        .map(|x| {
            x.clone()
                .split("")
                .filter(|x| {
                    *x == "0" || *x == "1" || *x == "2"
                })
                .map(|x| {
                    return match x {
                        "0" => Add,
                        "1" => Multiply,
                        "2" => Concat,
                        _ => CalculOperator::Unknown
                    }
                })
                .collect()
        })
        .collect()
}

fn get_all_operators_combinations(operation_members: &Vec<u64>, how_many_calcul_operator: u32) -> Vec<String> {
    let how_many_operators = operation_members.len() - 1;
    
    let mut counter = 0;
    let mut combinations = vec![];

    while counter != how_many_calcul_operator.pow(how_many_operators as u32) {
        let combination: String = format_radix(counter, how_many_calcul_operator);
        let combination_with_right_length = pad_left_with_char(&*combination, how_many_operators);
        combinations.push(combination_with_right_length);
        
        counter = counter + 1;
    }

    combinations
}

fn pad_left_with_char(s: &str, width: usize) -> String {
    let pad_len = width.saturating_sub(s.chars().count());
    format!("{}{}", '0'.to_string().repeat(pad_len), s)
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m.try_into().unwrap(), radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn parse_string_to_operations(string: String) -> Vec<(u64, Vec<u64>)> {
    string.lines().map(|line| {
        let split_line: Vec<&str> = line.split(':').collect();
        let split_numbers = split_line.get(1).unwrap();
        let operations: Vec<u64> = split_numbers.split(' ')
            .map(|x| -> u64 {
                x.parse().unwrap_or_default()
            })
            .filter(|x| *x != 0)
            .collect();
        (split_line.get(0).unwrap().parse().unwrap(), operations)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_day_7_a() {
        static TEST_PATH: &str = "src/day_7/test_data.txt";
        assert_eq!(run_a(TEST_PATH).unwrap(), 3749);
    }

    #[test]
    fn test_calculate_day_7_b() {
        static TEST_PATH: &str = "src/day_7/test_data.txt";
        assert_eq!(run_b(TEST_PATH).unwrap(), 11387);
    }
    
    // PARSE
    #[test]
    fn test_parse_to_operations() {
        let string_to_parse = "190: 10 19\n3267: 81 40 27";
        assert_eq!(
            parse_string_to_operations(string_to_parse.to_string()),
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27])
            ]
        );
    }

    // GET ALL COMBINATIONS
    #[test]
    fn test_get_binaries_string_2() {
        assert_eq!(
            get_all_operators_combinations(&vec![0, 1], 2),
            vec!["0", "1"]
        );
    }

    #[test]
    fn test_get_binaries_string_2_with_concat_operator() {
        assert_eq!(
            get_all_operators_combinations(&vec![0, 1], 3),
            vec!["0", "1", "2"]
        );
    }

    #[test]
    fn test_get_binaries_string_3_with_concat_operator() {
        assert_eq!(
            get_all_operators_combinations(&vec![0, 1, 2], 3),
            vec!["00", "01", "02", "10", "11", "12", "20", "21", "22"]
        );
    }

    #[test]
    fn test_get_binaries_string_3() {
        assert_eq!(
            get_all_operators_combinations(&vec![0, 1, 2], 2),
            vec!["00", "01", "10", "11"]
        );
    }

    #[test]
    fn test_get_binaries_string_4() {
        assert_eq!(
            get_all_operators_combinations(&vec![0, 1, 2, 3], 2),
            vec![
                "000", "001", "010", "011",
                "100", "101", "110", "111"
            ]
        );
    }

    // TRANSFORM_TO_CALCUL_OPERATYORS_COMBINATIONS
    #[test]
    fn test_fuck_off() {
        let string_to_parse = vec!["001".to_string(), "101".to_string()];
        assert_eq!(
            transform_to_calcul_operators_combinations(string_to_parse),
            vec![
                vec![Add, Add, Multiply],
                vec![Multiply, Add, Multiply]
            ]
        );
    }
    
    // CREATE_OPERATIONS_TO_APPLY
    #[test]
    fn test_calculate_operations_to_do_2_numbers() {
        let numbers : Vec<u64> = vec![0, 1];
        assert_eq!(create_operations_to_apply(&numbers, 2),
                   vec![
                       vec![Add],
                       vec![Multiply],
                   ]
        );
    }
    
    #[test]
    fn test_calculate_operations_to_do_3_numbers() {
        let numbers : Vec<u64> = vec![0, 1, 2];
        assert_eq!(create_operations_to_apply(&numbers, 2),
                   vec![
                vec![Add, Add],
                vec![Add, Multiply],
                vec![Multiply, Add],
                vec![Multiply, Multiply],
            ]
        );
    }
    
    #[test]
    fn test_calculate_operations_to_do_4_numbers() {
        let numbers : Vec<u64> = vec![0, 1, 2, 3];
        assert_eq!(create_operations_to_apply(&numbers, 2),
                   vec![
                       vec![Add, Add, Add],
                       vec![Add, Add, Multiply],
                       vec![Add, Multiply, Add],
                       vec![Add, Multiply, Multiply],
                       vec![Multiply, Add, Add],
                       vec![Multiply, Add, Multiply],
                       vec![Multiply, Multiply, Add],
                       vec![Multiply, Multiply, Multiply],
                   ]
        );
    }

}
