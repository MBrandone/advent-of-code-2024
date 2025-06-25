pub mod day_5 {
    use std::fs;

    static PATH: &str = "src/day_5/data.txt";
    // static TEST_PATH: &str = "src/day_5/test_data.txt";

    pub async fn day_5() {
        let graph_string = fs::read_to_string(PATH).unwrap();

        // SplitN est vraiment la meilleure fonction ?
        let mut divided_input = graph_string.splitn(2, "\n\n");
        let rules_input: String = divided_input.next().unwrap_or("").parse().unwrap();
        let lines_to_check_input: String = divided_input.next().unwrap_or("").parse().unwrap();

        let rules = convert_to_tuples(rules_input);
        let checks = convert_to_vec(lines_to_check_input);

        let right_lines: Vec<Vec<i32>> = checks.clone()
            .into_iter()
            .filter(|line| does_follow_rules(line, &rules))
            .collect();

        let sum = sum_of_middle_numbers(&right_lines);
        println!("La somme du chiffre du milieu des bonnes lignes est {}", sum);

        let wrong_lines: Vec<&Vec<i32>> = checks
            .iter()
            .filter(|&line| !does_follow_rules(&line, &rules))
            .collect();

        let corrected_lines: Vec<Vec<i32>> = wrong_lines
            .into_iter()
            .map(|line| reorder_line(line, &rules))
            .collect();
        
        let sum_corrected = sum_of_middle_numbers(&corrected_lines);
        
        println!("La somme des corrigés est {}", sum_corrected);
    }

    fn sum_of_middle_numbers(lines: &Vec<Vec<i32>>) -> i32 {
        let middle_number_from_each_corrected_lines : Vec<&i32> = lines
            .iter()
            .map(|line| line.get(line.len() / 2 ).unwrap_or(&0))
            .collect();

        middle_number_from_each_corrected_lines.into_iter().sum()
    }

    fn convert_to_tuples(rules_string: String) -> Vec<(i32, i32)> {
        let mut rules_tuples = Vec::new();
        for line in rules_string.lines() {
            let mut two_members = line.splitn(2, "|");
            let first_member = two_members.next().unwrap().parse().unwrap();
            let second_member = two_members.next().unwrap().parse().unwrap();
            let tuple: (i32, i32) = (first_member, second_member);
            rules_tuples.push(tuple);
        }

        rules_tuples
    }

    fn convert_to_vec(checks_string: String) -> Vec<Vec<i32>> {
        let mut checks: Vec<Vec<i32>> = Vec::new();

        for line in checks_string.lines() {
            let mut check: Vec<i32> = Vec::new();
            for number in line.split(",") {
                check.push(number.parse().unwrap())
            }
            checks.push(check);
        }

        checks
    }

    fn does_follow_rules(line_to_check: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
        for (index, number_to_check) in line_to_check.iter().enumerate() {
            let numbers_to_compare: Vec<i32> = line_to_check[index..].to_vec();
            for number_to_compare in numbers_to_compare {
                let unrespected_rule = rules.iter().find(|&&x| x == (number_to_compare, *number_to_check));

                match unrespected_rule {
                    Some(_) => return false,
                    None => ()
                }

            }
        }

        true
    }

    fn reorder_line(line: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
        for (index, number_to_check) in line.iter().enumerate() {
            let numbers_to_compare: Vec<i32> = line[index..].to_vec();
            for number_to_compare in numbers_to_compare {
                let unrespected_rule = rules.iter().find(|&&x| x == (number_to_compare, *number_to_check));
                
                match unrespected_rule {
                    Some(_) => {
                        let index_to_insert = index;
                        let mut new_line : Vec<i32> = line.clone();
                        new_line.retain(|x| *x != number_to_compare);
                        new_line.insert(index_to_insert, number_to_compare);
                        return reorder_line(&new_line, rules);
                    }
                    None => ()
                }

            }
        }

        line.clone()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_rules_string_to_tuples() {
            let rules_string: String = "47|53\n97|13\n97|61\n97|47".parse().unwrap();
            assert_eq!(
                convert_to_tuples(rules_string),
                vec!((47, 53), (97, 13), (97, 61), (97, 47))
            );
        }

        #[test]
        fn test_convert_checks_to_vec() {
            let check_string: String = "75,47,61,53,29\n97,61,53,29,13\n75,29,13".parse().unwrap();
            assert_eq!(
                convert_to_vec(check_string),
                vec!(
                    vec!(75, 47, 61, 53, 29),
                    vec!(97, 61, 53, 29, 13),
                    vec!(75, 29, 13)
                )
            );
        }

        #[test]
        fn test_respect_rules() {
            let line_to_check = vec!(75, 47, 61, 53, 29);
            let rules = vec![(75, 47), (47, 61), (53, 29)];
            assert_eq!(
                does_follow_rules(&line_to_check, &rules),
                true
            );
        }

        #[test]
        fn test_respect_rules_1() {
            let line_to_check = vec!(1, 2, 3);
            let rules = vec![(1, 2), (3, 1), (2, 3)];
            assert_eq!(
                does_follow_rules(&line_to_check, &rules),
                false
            );
        }

        #[test]
        fn test_reorder_following_rules() {
            let line_to_check = vec!(1, 3, 2, 5, 4);
            let rules = vec![(1, 2), (2, 3), (3, 4), (4, 5)];
            assert_eq!(
                reorder_line(&line_to_check, &rules),
                vec!(1, 2, 3, 4, 5)
            );
        }

    }
}
