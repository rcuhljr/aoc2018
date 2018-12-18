use super::utility;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;

pub fn solve_a() -> String {
    score_after(&parse_input("input18.txt".to_string()), 10)
        .0
        .to_string()
}

pub fn solve_b() -> String {
    sustainability("input18.txt".to_string()).to_string()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn up(self) -> Point {
        Point::new(self.x, self.y - 1)
    }
    fn left(self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    fn right(self) -> Point {
        Point::new(self.x + 1, self.y)
    }
    fn down(self) -> Point {
        Point::new(self.x, self.y + 1)
    }
}

fn parse_input(filename: String) -> Vec<Vec<char>> {
    let raw_data = utility::load_strings(filename);
    let mut field: Vec<Vec<char>> = Vec::new();

    for line in raw_data.iter() {
        field.push(line.chars().collect());
    }

    field
}

fn get_point(point: Point, field: &Vec<Vec<char>>) -> char {
    let (width, height) = (field[0].len() as i32, field.len() as i32);

    if point.x < 0 || point.x >= width || point.y < 0 || point.y >= height {
        '.'
    } else {
        field[point.y as usize][point.x as usize]
    }
}

fn count_neighbors(point: Point, field: &Vec<Vec<char>>) -> (usize, usize) {
    let mut tree = 0;
    let mut lumber = 0;
    let neighbors = vec![
        point.up(),
        point.up().right(),
        point.right(),
        point.right().down(),
        point.down(),
        point.down().left(),
        point.left(),
        point.left().up(),
    ];

    neighbors
        .iter()
        .map(|x| get_point(*x, &field))
        .for_each(|space| {
            if space == '|' {
                tree += 1
            } else if space == '#' {
                lumber += 1
            }
        });

    (tree, lumber)
}

fn score_after(field: &Vec<Vec<char>>, rounds: usize) -> (i32, Vec<Vec<char>>) {
    let mut field = field.clone();
    let (width, height) = (field[0].len() as i32, field.len() as i32);

    for _ in 0..rounds {
        let mut new_field: Vec<Vec<char>> = vec![];
        for y in 0..height {
            let mut new_row: Vec<char> = vec![];
            for x in 0..width {
                let neighbors = count_neighbors(Point::new(x, y), &field);
                let current = get_point(Point::new(x, y), &field);
                match current {
                    '|' => if neighbors.1 >= 3 {
                        new_row.push('#')
                    } else {
                        new_row.push('|')
                    },
                    '#' => if neighbors.1 >= 1 && neighbors.0 >= 1 {
                        new_row.push('#')
                    } else {
                        new_row.push('.')
                    },
                    '.' => if neighbors.0 >= 3 {
                        new_row.push('|')
                    } else {
                        new_row.push('.')
                    },
                    _ => panic!("what?"),
                }
            }
            new_field.push(new_row);
        }
        field = new_field;
    }
    let mut tree = 0;
    let mut lumber = 0;
    for y in 0..height {
        for x in 0..width {
            let current = get_point(Point::new(x, y), &field);
            match current {
                '|' => tree += 1,
                '#' => lumber += 1,
                _ => (),
            }
        }
    }
    (tree * lumber, field.to_vec())
}

fn sustainability(filename: String) -> i32 {
    let mut starter = parse_input(filename);
    let mut been_there: Vec<i32> = vec![];
    let endgoal = 1000000000;
    for round in 0..endgoal {
        let pair = score_after(&starter, 1);
        if round >= 1000 {
            if been_there.contains(&pair.0) {
                let last_index = been_there.iter().position(|x| *x == pair.0).unwrap();
                let cycle = been_there.len() - last_index;
                been_there.push(pair.0.clone());
                return been_there[last_index + (endgoal - 1000 - been_there.len()) % cycle];
            } else {
                been_there.push(pair.0);
            }
        }
        starter = pair.1;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_sustainable() {
        let actual = sustainability("input18.txt".to_string());

        assert_eq!(actual, 195305);
    }

    #[test]
    fn should_solve_ten_min_sample() {
        let actual = score_after(&parse_input("./src/day18/test.txt".to_string()), 10);

        assert_eq!(actual.0, 1147);
    }

    #[test]
    fn should_solve_two_five_sample() {
        let mut starter = parse_input("./src/day18/test.txt".to_string());
        let mut actual = score_after(&starter, 5);
        let mut actual = score_after(&actual.1, 5);

        assert_eq!(actual.0, 1147);
    }

    #[test]
    fn should_parse_sample_forest() {
        let actual: Vec<Vec<char>> = parse_input("./src/day18/test.txt".to_string());

        assert_eq!(actual.len(), 10);
    }

    #[test]
    fn should_count_neighbors() {
        let field: Vec<Vec<char>> = parse_input("./src/day18/test.txt".to_string());
        assert_eq!(count_neighbors(Point::new(0, 0), &field), (0, 1));

        assert_eq!(count_neighbors(Point::new(9, 9), &field), (2, 0));

        assert_eq!(count_neighbors(Point::new(8, 3), &field), (2, 3));
    }

}
