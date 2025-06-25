/*
Apprentissage
- Algo de Floyd pour trouver des cycles

*/

use std::cmp::PartialEq;
use std::fs;
use std::io::Error;

static PATH: &str = "src/day_6/data.txt";
//static TEST_PATH: &str = "src/day_6/test_data.txt";

const OBSTRUCTION: char = '#';
const TEMPORARY_OBSTRUCTION: char = 'O';
const OPEN_ROAD: char = '.';
const SHIP_PATH: char = 'X';

const SHIP_GOING_TOP: char = '^';
const SHIP_GOING_DOWN: char = 'v';
const SHIP_GOING_LEFT: char = '<';
const SHIP_GOING_RIGHT: char = '>';

const SHIP_SIGNS: [char; 4] = [
    SHIP_GOING_TOP,
    SHIP_GOING_DOWN,
    SHIP_GOING_LEFT,
    SHIP_GOING_RIGHT,
];

pub async fn day_6() -> Result<i32, Error> {
    let map_string = fs::read_to_string(PATH)?;

    // A
    let mut map: Map = Map::new(&map_string);

    while !map.ship_is_going_out() {
        map.go_to_next_position();
    }

    println!("Il y a {} points passés", map.count_points_passed_by());

    // B
    // ALGO DE FLOYD https://en.wikipedia.org/wiki/Cycle_detection
    // => S'il y a un cycle, deux vaisseaux allant à une vitesse différente se retrouveront au même endroit
    let mut temporary_obstruction_that_cause_a_cycle: Vec<(usize, usize)> = vec![];

    for index_j in 0..map.map.len() {
        for index_i in 0..map.map.len() {
            println!("Obstruction temporaire {}, {}", index_i, index_j);

            if index_i == map.ship_position.x.into() && index_j == map.ship_position.y.into()
                || map.map[index_j][index_i] == OBSTRUCTION
            {
                continue;
            }

            let mut tortoise_map = Map::new(&map_string);
            let mut hare_map = Map::new(&map_string);

            let previous_sign = map.map[index_j][index_i];
            tortoise_map.map[index_j][index_i] = TEMPORARY_OBSTRUCTION;
            hare_map.map[index_j][index_i] = TEMPORARY_OBSTRUCTION;

            let mut move_counter = 0;
            while !hare_map.ship_is_going_out() {
                move_counter += 1;

                hare_map.go_to_next_position();
                if move_counter % 2 == 0 {
                    tortoise_map.go_to_next_position()
                }

                if hare_map.ship_position == tortoise_map.ship_position {
                    println!(
                        "Position d'obstruction provoquant un cycle en {}, {}",
                        index_i, index_j
                    );
                    temporary_obstruction_that_cause_a_cycle.push((index_i, index_j));
                    break;
                }
            }

            tortoise_map.map[index_j][index_i] = previous_sign;
            hare_map.map[index_j][index_i] = previous_sign;
        }
    }

    println!(
        "Les obstruction temporaires sont {:?}",
        temporary_obstruction_that_cause_a_cycle
    );
    println!(
        "Il y a {} obstructions temporaire qui cause un cycle",
        temporary_obstruction_that_cause_a_cycle.len()
    );

    Ok(0)
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    direction: char,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction && self.x == other.x && self.y == other.y
    }
}

struct Map {
    pub map: Vec<Vec<char>>,
    path_visualisation: Vec<Vec<char>>,
    pub ship_position: Position,
}

impl Map {
    fn new(map_string: &String) -> Map {
        let map: Vec<Vec<char>> = map_string.lines().map(|l| l.chars().collect()).collect();

        Map {
            map: map.clone(),
            path_visualisation: map
                .clone()
                .into_iter()
                .map(|line| {
                    line.into_iter()
                        .map(|x| {
                            return if SHIP_SIGNS.contains(&x) {
                                SHIP_PATH
                            } else {
                                x
                            };
                        })
                        .collect()
                })
                .collect(),
            ship_position: Self::find_ship_position(&map),
        }
    }

    fn find_ship_position(map: &Vec<Vec<char>>) -> Position {
        let mut x = 0;
        let mut y = 0;
        let mut direction = '^';

        for index_j in 0..map.len() {
            for index_i in 0..map.len() {
                if SHIP_SIGNS.contains(&map[index_j][index_i]) {
                    x = index_i.try_into().unwrap();
                    y = index_j.try_into().unwrap();
                    direction = map[index_j][index_i];
                }
            }
        }

        Position { x, y, direction }
    }

    fn ship_is_going_out(&self) -> bool {
        match self.ship_position.direction {
            SHIP_GOING_TOP => return self.ship_position.y == 0,
            SHIP_GOING_DOWN => return self.ship_position.y == self.map.len() - 1,
            SHIP_GOING_RIGHT => return self.ship_position.x == self.map.len() - 1,
            SHIP_GOING_LEFT => return self.ship_position.x == 0,
            _ => {}
        }
        false
    }

    pub fn go_to_next_position(&mut self) {
        self.map[self.ship_position.y][self.ship_position.x] = OPEN_ROAD;

        // Prendre des déplacements en fonctions de la direction
        //let mut direction = (0, 0);
        //match self.ship_position.direction {
        //    SHIP_GOING_TOP => {
        //        direction = (0, -1);
        //    }
        //    SHIP_GOING_DOWN => {
        //        direction = (0, 1);
        //    }
        //    SHIP_GOING_RIGHT => {
        //        direction = (1, 0);
        //    }
        //    SHIP_GOING_LEFT => {
        //        direction = (-1, 0);
        //    }
        //    _ => {}
        //}

        // ça divise par 4 le code si on utilise les directions
        // Tenter de faire le virage + le déplacement direct ?
        match self.ship_position.direction {
            SHIP_GOING_TOP => {
                if self.map[self.ship_position.y - 1][self.ship_position.x]
                    == OBSTRUCTION
                    || self.map[self.ship_position.y - 1][self.ship_position.x]
                        == TEMPORARY_OBSTRUCTION
                {
                    self.map[self.ship_position.y][self.ship_position.x] =
                        SHIP_GOING_RIGHT;
                    self.ship_position = Position {
                        x: self.ship_position.x,
                        y: self.ship_position.y,
                        direction: SHIP_GOING_RIGHT,
                    }
                }
                if self.map[self.ship_position.y - 1][self.ship_position.x]
                    == OPEN_ROAD
                {
                    self.map[self.ship_position.y - 1][self.ship_position.x] =
                        SHIP_GOING_TOP;
                    self.ship_position = Position {
                        x: self.ship_position.x,
                        y: self.ship_position.y - 1,
                        direction: SHIP_GOING_TOP,
                    }
                }
            }
            SHIP_GOING_DOWN => {
                if self.map[self.ship_position.y + 1][self.ship_position.x]
                    == OBSTRUCTION
                    || self.map[self.ship_position.y + 1][self.ship_position.x]
                        == TEMPORARY_OBSTRUCTION
                {
                    self.map[self.ship_position.y][self.ship_position.x] =
                        SHIP_GOING_LEFT;
                    self.ship_position = Position {
                        x: self.ship_position.x,
                        y: self.ship_position.y,
                        direction: SHIP_GOING_LEFT,
                    }
                }
                if self.map[self.ship_position.y + 1][self.ship_position.x]
                    == OPEN_ROAD
                {
                    self.map[self.ship_position.y + 1][self.ship_position.x] =
                        SHIP_GOING_DOWN;
                    self.ship_position = Position {
                        x: self.ship_position.x,
                        y: self.ship_position.y + 1,
                        direction: SHIP_GOING_DOWN,
                    }
                }
            }
            SHIP_GOING_RIGHT => {
                if self.map[self.ship_position.y][self.ship_position.x + 1]
                    == OBSTRUCTION
                    || self.map[self.ship_position.y][self.ship_position.x + 1]
                        == TEMPORARY_OBSTRUCTION
                {
                    self.map[self.ship_position.y][self.ship_position.x] =
                        SHIP_GOING_DOWN;
                    self.ship_position = Position {
                        x: self.ship_position.x,
                        y: self.ship_position.y,
                        direction: SHIP_GOING_DOWN,
                    }
                }
                if self.map[self.ship_position.y][self.ship_position.x + 1]
                    == OPEN_ROAD
                {
                    self.map[self.ship_position.y][self.ship_position.x + 1] =
                        SHIP_GOING_RIGHT;
                    self.ship_position = Position {
                        x: self.ship_position.x + 1,
                        y: self.ship_position.y,
                        direction: SHIP_GOING_RIGHT,
                    }
                }
            }
            SHIP_GOING_LEFT => {
                if self.map[self.ship_position.y][self.ship_position.x - 1]
                    == OBSTRUCTION
                    || self.map[self.ship_position.y][self.ship_position.x - 1]
                        == TEMPORARY_OBSTRUCTION
                {
                    self.map[self.ship_position.y][self.ship_position.x] =
                        SHIP_GOING_TOP;
                    self.ship_position = Position {
                        x: self.ship_position.x,
                        y: self.ship_position.y,
                        direction: SHIP_GOING_TOP,
                    }
                }
                if self.map[self.ship_position.y][self.ship_position.x - 1]
                    == OPEN_ROAD
                {
                    self.map[self.ship_position.y][self.ship_position.x - 1] =
                        SHIP_GOING_LEFT;
                    self.ship_position = Position {
                        x: self.ship_position.x - 1,
                        y: self.ship_position.y,
                        direction: SHIP_GOING_LEFT,
                    }
                }
            }
            _ => {}
        }

        self.update_path_visualisation_sign();
    }

    fn update_path_visualisation_sign(&mut self) {
        self.path_visualisation[self.ship_position.y][self.ship_position.x] =
            SHIP_PATH;
    }

    pub fn count_points_passed_by(&self) -> u32 {
        self.path_visualisation
            .iter()
            .flatten()
            .map(|x| return if *x == 'X' { 1 } else { 0 })
            .sum()
    }

    //pub fn print_visualisation_map(&self) {
    //    println!("la map de trajectoire");
    //    for line in &self.path_visualisation {
    //        println!("{:?}", line);
    //    }
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_map_open_road() {
        let mut map = Map::new(&"...\n.^.\n...\n".to_string());
        map.go_to_next_position();
        assert_eq!(
            map.map,
            vec![
                vec!['.', '^', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.']
            ]
        )
    }

    #[test]
    fn test_update_map_with_obstruction() {
        let mut map = Map::new(&".#.\n.^.\n...\n".to_string());
        map.go_to_next_position();
        assert_eq!(
            map.map,
            vec![
                vec!['.', '#', '.'],
                vec!['.', '>', '.'],
                vec!['.', '.', '.']
            ]
        )
    }

    #[test]
    fn test_update_map_without_obstruction() {
        let mut map = Map::new(&"...\n.<.\n...\n".to_string());
        map.go_to_next_position();
        assert_eq!(
            map.map,
            vec![
                vec!['.', '.', '.'],
                vec!['<', '.', '.'],
                vec!['.', '.', '.']
            ]
        )
    }

    #[test]
    fn test_ended_not() {
        let map = Map::new(&"...\n.<.\n...".to_string());
        assert_eq!(map.ship_is_going_out(), false)
    }

    #[test]
    fn test_not_ended_by_left() {
        let map = Map::new(&"...\n^..\n...".to_string());
        assert_eq!(map.ship_is_going_out(), false)
    }

    #[test]
    fn test_ended_by_left() {
        let map = Map::new(&"...\n<..\n...".to_string());
        assert_eq!(map.ship_is_going_out(), true)
    }

    #[test]
    fn test_ended_by_top() {
        let map = Map::new(&"^..\n...\n...".to_string());
        assert_eq!(map.ship_is_going_out(), true)
    }

    #[test]
    fn test_ended_by_bottom() {
        let map = Map::new(&"...\n...\n.v.".to_string());
        assert_eq!(map.ship_is_going_out(), true)
    }

    #[test]
    fn test_count_points_passed_by() {
        let map = Map::new(&"XXX\nX..\nX..".to_string());
        assert_eq!(map.count_points_passed_by(), 5)
    }
}
