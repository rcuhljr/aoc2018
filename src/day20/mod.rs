use super::utility;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve_a() -> String {
    let mut maze = HashSet::new();
    build_maze(
        Point::new(0, 0),
        &parse_input("input20.txt".to_string()),
        &mut maze,
    );

    count_longest_path(&maze).0.to_string()
}

pub fn solve_b() -> String {
    let mut maze = HashSet::new();
    build_maze(
        Point::new(0, 0),
        &parse_input("input20.txt".to_string()),
        &mut maze,
    );

    count_longest_path(&maze).1.to_string()
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

    fn e(self) -> Point {
        Point::new(self.x + 1, self.y)
    }
    fn w(self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    fn n(self) -> Point {
        Point::new(self.x, self.y + 1)
    }
    fn s(self) -> Point {
        Point::new(self.x, self.y - 1)
    }
}

fn parse_input(filename: String) -> Vec<char> {
    let raw_data = utility::load_strings(filename);
    let raw_string: Vec<char> = raw_data.iter().cloned().next().unwrap().chars().collect();
    raw_string[1..raw_string.len() - 1].to_vec()
}

fn count_longest_path(maze: &HashSet<(Point, Point)>) -> (usize, usize) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut todo: VecDeque<(Point, usize)> = VecDeque::new();
    let mut max_depth = 0;
    let mut far_away_rooms = 0;

    todo.push_back((Point::new(0, 0), 0));

    while todo.len() > 0 {
        let (current, depth) = todo.pop_front().unwrap();
        if depth > max_depth {
            max_depth = depth;
        }
        let mut children = vec![
            (current, current.e()),
            (current, current.n()),
            (current, current.w()),
            (current, current.s()),
        ];
        children = children
            .iter()
            .filter(|x| !visited.contains(&x.1) && maze.contains(x))
            .cloned()
            .collect();
        if depth >= 1000 {
            far_away_rooms += 1;
        }
        if children.len() == 0 {
            continue;
        }
        for child in children {
            todo.push_back((child.1, depth + 1));
        }
        visited.insert(current.clone());
    }

    (max_depth, far_away_rooms)
}

fn add_simple_path(
    origin: Point,
    directions: &[char],
    maze: &mut HashSet<(Point, Point)>,
) -> Point {
    let mut current = origin.clone();
    for step in directions.iter() {
        match step {
            'E' => {
                maze.insert((current, current.e()));
                current = current.e();
            }
            'S' => {
                maze.insert((current, current.s()));
                current = current.s();
            }
            'N' => {
                maze.insert((current, current.n()));
                current = current.n();
            }
            'W' => {
                maze.insert((current, current.w()));
                current = current.w();
            }
            _ => panic!("go away"),
        }
    }
    current
}

fn build_maze(origin: Point, directions: &[char], maze: &mut HashSet<(Point, Point)>) {
    if directions.len() == 0 {
        return;
    }
    let current: Vec<char> = directions
        .iter()
        .take_while(|x| **x != '|' && **x != '(')
        .cloned()
        .collect();

    if current.len() == directions.len() {
        add_simple_path(origin, &current, maze);
        return;
    }

    let mut paren_count = 0;
    let mut split_indexes = vec![];
    directions.iter().enumerate().for_each(|pair| {
        let (index, val) = pair;
        if paren_count > 0 {
            match val {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                _ => (),
            }
        } else {
            match val {
                '(' => paren_count += 1,
                '|' => split_indexes.push(index),
                _ => (),
            }
        }
    });
    if split_indexes.len() > 0 {
        let mut start = 0;
        split_indexes.iter().for_each(|end| {
            build_maze(origin, &directions[start..*end], maze);
            start = *end;
        });
        build_maze(origin, &directions[start + 1..], maze);
        return;
    }

    let mut paren_count = 0;
    let mut end_pos = directions.len();
    for (index, val) in directions.iter().skip(current.len()).enumerate() {
        if paren_count > 0 {
            match val {
                '(' => paren_count += 1,
                ')' => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        end_pos = current.len() + index + 1;
                        break;
                    }
                }
                _ => (),
            }
        } else {
            match val {
                '(' => paren_count += 1,
                _ => (),
            }
        }
    }

    let current_point = add_simple_path(origin, &current, maze);
    build_maze(
        current_point,
        &directions[current.len() + 1..end_pos - 1],
        maze,
    );
    if end_pos != directions.len() {
        build_maze(current_point, &directions[end_pos..], maze);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_initial_input20() {
        let actual = parse_input("./src/day20/test.txt".to_string());

        assert_eq!(
            actual,
            [
                'E', 'N', 'W', 'W', 'W', '(', 'N', 'E', 'E', 'E', '|', 'S', 'S', 'E', '(', 'E',
                'E', '|', 'N', ')', ')'
            ]
        );
    }

    #[test]
    fn should_solve_sample20() {
        let mut maze = HashSet::new();
        build_maze(
            Point::new(0, 0),
            &parse_input("./src/day20/test.txt".to_string()),
            &mut maze,
        );

        println!("{:?}", maze);

        let actual = count_longest_path(&maze);

        assert_eq!(actual.0, 10);

        maze = HashSet::new();
        build_maze(
            Point::new(0, 0),
            &parse_input("./src/day20/test2.txt".to_string()),
            &mut maze,
        );

        let actual = count_longest_path(&maze);

        assert_eq!(actual.0, 23);

        maze = HashSet::new();
        build_maze(
            Point::new(0, 0),
            &parse_input("./src/day20/test3.txt".to_string()),
            &mut maze,
        );

        let actual = count_longest_path(&maze);

        assert_eq!(actual.0, 31);

        maze = HashSet::new();
        build_maze(
            Point::new(0, 0),
            &parse_input("./src/day20/test4.txt".to_string()),
            &mut maze,
        );

        let actual = count_longest_path(&maze);

        assert_eq!(actual.0, 18);
    }

}
