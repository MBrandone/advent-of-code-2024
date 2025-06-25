/*
Enseignement
On est allé vers un graph mais un Vec de Vec aurait pu suffire
Problématique à comprendre sur les références dans les if let Some()
*/
pub mod day_4 {
    use std::fs;

    static PATH: &str = "src/day_4/data.txt";
    type Coord = (usize, usize);
    static DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    pub async fn day_4() -> i32 {
        let graph_string = fs::read_to_string(PATH).unwrap();
        
        let lines: Vec<Vec<char>> = graph_string.lines().map(|l| l.chars().collect()).collect();
        let rows = lines.len();
        let cols = lines[0].len();
        
        count_xmas(rows, cols, &lines);
        count_x_mas(rows, cols, &lines)
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Word {
        first: char, 
        second: char, 
        third: char, 
        fourth: char
    }
    
    impl Word {
        fn is_xmas(&self)-> bool {
            self.first == 'X' && self.second == 'M' && self.third == 'A' && self.fourth == 'S'
        }    
    }

    fn add_usize_isize(base: usize, offset: isize) -> Option<usize> {
        if offset >= 0 {
            base.checked_add(offset as usize)
        } else {
            base.checked_sub(offset.unsigned_abs())
        }
    }
    
    fn build_word(lines: &Vec<Vec<char>>, (x, y): Coord, direction: (isize, isize))-> Option<Word> {
        if x < lines.len() 
            && x as isize + direction.0 >= 0 && x as isize + direction.0 < lines.len() as isize 
            && x as isize + 2 * direction.0 >= 0 && x as isize + 2 * direction.0 < lines.len() as isize 
            && x as isize + 3 * direction.0 >= 0 && x as isize + 3 * direction.0 < lines.len() as isize
            && y < lines.len() 
            && y as isize + direction.1 >= 0 && y as isize + direction.1 < lines.len() as isize 
            && y as isize + 2 * direction.1 >= 0 && y as isize + 2 * direction.1 < lines.len() as isize 
            && y as isize + 3 * direction.1 >= 0 && y as isize + 3 * direction.1 < lines.len() as isize 
        { 
            if let Some(&first) = lines.get(x).and_then(|r| r.get(y)) {
                if let Some(&second) = lines.get(add_usize_isize(x, direction.0).unwrap()).and_then(|r| r.get(add_usize_isize(y, direction.1).unwrap())) {
                    if let Some(&third) = lines.get(add_usize_isize(x, 2 * direction.0).unwrap()).and_then(|r| r.get(add_usize_isize(y, 2 * direction.1).unwrap())) {
                        if let Some(&fourth) = lines.get(add_usize_isize(x, 3 * direction.0).unwrap()).and_then(|r| r.get(add_usize_isize(y, 3 * direction.1).unwrap())) {
                            return Some(Word {
                                first,
                                second,
                                third,
                                fourth
                            })
                        }
                    }
                }
            }
            
        }
        None
    }

    fn count_xmas(rows: usize, cols: usize, lines: &Vec<Vec<char>>) -> i32 {
        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                let current_pos = (i, j);
                let value = lines[i][j];
            
                if value == 'X' {
                for direction in DIRECTIONS {
                    if let Some(word) = build_word(lines, current_pos, direction) {
                        if word.is_xmas() {  
                            count = count + 1
                        }
                    }
                }
            }
            }
        }

        println!("{}", count);
        count
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    struct XmasShape {
        center: char,
        top_left: char,
        bottom_left: char,
        top_right: char,
        bottom_right: char,
    }

    impl XmasShape {
        fn two_adjacent_m_and_s(&self) -> bool {
            let letters = vec![&self.top_left, &self.top_right, &self.bottom_right, &self.bottom_left];
            let how_many_m = letters.iter().filter(|&&x| *x == 'M').count();
            let how_many_s = letters.iter().filter(|&&x| *x == 'S').count();
            let two_adjacent_letters =  &self.top_left == &self.bottom_left || &self.top_left == &self.top_right;

            self.center == 'A' && how_many_m == 2 && how_many_s == 2 && two_adjacent_letters
        }
    }
    
    fn build_xmas_shape(lines: &Vec<Vec<char>>, (x, y): Coord) -> Option<XmasShape> {
        if x == 0 || y == 0 || x == lines.len() - 1 || y == lines.len() - 1 { 
            return None
        }
        let center = lines[x][y];
        let top_left = lines[x - 1][y - 1];
        let bottom_left = lines[x + 1][y - 1];
        let top_right = lines[x - 1][y + 1];
        let bottom_right = lines[x + 1][y + 1];
        
        Some(XmasShape {
            center,
            top_left,
            bottom_left,
            top_right,
            bottom_right,
        })
    }
    
    fn count_x_mas(rows: usize, cols: usize, lines: &Vec<Vec<char>>)-> i32 {
        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                if let Some(x_mas_shape) = build_xmas_shape(&lines, (i, j)) {
                    if x_mas_shape.two_adjacent_m_and_s() {
                        count = count + 1;
                    }
                }
            }
        }

        println!("count X-MAS {}", count);
        count
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_read_file() {
            assert_eq!(
                fs::read_to_string("src/day_4/test_data.txt").unwrap(),
                "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"
            );
        }

        #[test]
        fn test_day_1_a() {
            let graph_string: String = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX".parse().unwrap();
            let lines: Vec<Vec<char>> = graph_string.lines().map(|l| l.chars().collect()).collect();
            let rows = lines.len();
            let cols = lines[0].len();
            assert_eq!(
                count_xmas(rows, cols, &lines),
                18
            );
        }

        #[test]
        fn test_day_1_b() {
            let graph_string: String = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX".parse().unwrap();
            let lines: Vec<Vec<char>> = graph_string.lines().map(|l| l.chars().collect()).collect();
            let rows = lines.len();
            let cols = lines[0].len();
            assert_eq!(
                count_x_mas(rows, cols, &lines),
                9
            );
        }

        #[test]
        fn test_build_x_mas_shape() {
            let lines: Vec<Vec<char>> = vec![vec!['M', 'A', 'S'], vec!['M', 'A', 'S'], vec!['M', 'A', 'S']];
            assert_eq!(
                build_xmas_shape(&lines, (1,1)),
                Some(XmasShape {
                    center: 'A',
                    top_left: 'M',
                    bottom_left: 'M',
                    top_right: 'S',
                    bottom_right: 'S',
                })
            );
        }

        #[test]
        fn test_build_x_mas_shape_not_possible() {
            let lines: Vec<Vec<char>> = vec![vec!['M', 'A', 'S'], vec!['M', 'A', 'S'], vec!['M', 'A', 'S']];
            assert_eq!(
                build_xmas_shape(&lines, (0,1)),
                None
            );
        }

        #[test]
        fn test_build_word_1() {
            let lines: Vec<Vec<char>> = vec![
                vec!['X', 'M', 'A', 'S'], 
                vec!['X', 'M', 'A', 'S'], 
                vec!['X', 'M', 'A', 'S'], 
                vec!['X', 'M', 'A', 'S']
            ];
            assert_eq!(
                build_word(&lines, (0,0), (0, 1)),
                Some(Word {
                    first: 'X',
                    second: 'M',
                    third: 'A',
                    fourth: 'S'
                })
            );
        }

        #[test]
        fn test_build_word_2() {
            let lines: Vec<Vec<char>> = vec![
                vec!['X', 'M', 'A', 'S'],
                vec!['X', 'M', 'A', 'S'],
                vec!['X', 'M', 'A', 'S'],
                vec!['X', 'M', 'A', 'S']
            ];
            assert_eq!(
                build_word(&lines, (0,1), (0, 1)),
                None
            );
        }

    }
}
