use std::fs;

const PATH: &'static str = "src/day_2/data.txt";

pub async fn day_2() {
    let buffer = fs::read_to_string(PATH).unwrap();
    let reports = parse_string_to_reports(buffer);

    let valid_reports_number = count_valid_reports(&reports);
    let valid_reports_number_if_one_item_removed =
        count_valid_reports_if_a_number_is_dropped(&reports);

    println!("There is {} valid reports", valid_reports_number);
    println!("There is {} valid reports if we remove one item", valid_reports_number_if_one_item_removed);
}

fn parse_string_to_reports(buffer: String) -> Vec<Vec<u32>> {
    let mut reports: Vec<Vec<u32>> = Vec::new();

    for line in buffer.lines() {
        reports.push(parse_line_to_report(line))
    }
    reports
}

fn parse_line_to_report(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn count_valid_reports(reports: &Vec<Vec<u32>>) -> usize {
    reports.iter().filter(|x| report_is_valid(&x)).count()
}

fn count_valid_reports_if_a_number_is_dropped(reports: &Vec<Vec<u32>>) -> usize {
    reports
        .iter()
        .filter(|x| report_is_valid_if_a_number_is_dropped(&x))
        .count()
}

fn report_is_valid(report: &Vec<u32>) -> bool {
    let variations: Vec<i32> = calculate_variations(report);

    let minimum_variation = variations.iter().map(|x| x.abs()).min().unwrap();
    let maximum_variation = variations.iter().map(|x| x.abs()).max().unwrap();
    
    let variations_are_beetween_1_and_3 = minimum_variation > 0 && maximum_variation <= 3;

    let report_numbers_are_only_increasing_or_only_decreasing = report_numbers_are_only_increasing_or_only_decreasing(variations);
    
    variations_are_beetween_1_and_3 && report_numbers_are_only_increasing_or_only_decreasing
}

fn report_numbers_are_only_increasing_or_only_decreasing(variations: Vec<i32>) -> bool {
    let increases = variations
        .iter()
        .filter(|&&x| x < 0)
        .count();
    let decreases = variations
        .iter()
        .filter(|&&x| x > 0)
        .count();
    
    increases == 0 && decreases != 0 || increases != 0 && decreases == 0 
}

fn calculate_variations(report: &Vec<u32>) -> Vec<i32> {
    let mut variations: Vec<i32> = vec![];
    
    for two_items in report.windows(2) {
        if let [first_item, second_item] = two_items {
            let difference = *first_item as i32 - *second_item as i32;

            variations.push(difference);
        }
    }
    
    variations
}

fn report_is_valid_if_a_number_is_dropped(report: &Vec<u32>) -> bool {
    if report_is_valid(report) {
        return true;
    }

    for (index, _value) in report.iter().enumerate() {
        let mut cloned_report = report.clone();
        cloned_report.remove(index);
        if report_is_valid(&cloned_report) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_valid_report() {
        assert_eq!(
            count_valid_reports(&vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]),
            2
        );
    }

    #[test]
    fn test_report_is_valid() {
        assert_eq!(report_is_valid(&vec![7, 6, 4, 2, 1]), true);
    }

    #[test]
    fn test_report_is_valid_2() {
        assert_eq!(report_is_valid(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_report_is_invalid_when_incresing_more_than_three() {
        assert_eq!(report_is_valid(&vec![1, 2, 7, 8, 9]), false);
    }

    #[test]
    fn test_report_is_invalid_when_decresing_more_than_three() {
        assert_eq!(report_is_valid(&vec![9, 7, 6, 2, 1]), false);
    }

    #[test]
    fn test_report_is_invalid_when_not_only_increasing_or_decreasing() {
        assert_eq!(report_is_valid(&vec![1, 3, 2, 4, 5]), false);
    }

    #[test]
    fn test_report_is_invalid_when_stable() {
        assert_eq!(report_is_valid(&vec![8, 6, 4, 4, 1]), false);
    }
}
