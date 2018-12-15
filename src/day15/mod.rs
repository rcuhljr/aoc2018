use super::utility;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve_a() -> String {
    find_end_score("input15.txt".to_string(), 3).0.to_string()
}

pub fn solve_b() -> String {
    find_flawless_victory("input15.txt".to_string()).to_string()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord)]
struct Unit {
    x: i32,
    y: i32,
    race: char,
    hp: i32,
    ap: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_blocked(&self, field: &Vec<Vec<bool>>, units: &Vec<Unit>) -> bool {
        !field[self.y as usize][self.x as usize] || units
            .iter()
            .any(|u| self.x == u.x && self.y == u.y && u.race != 'D')
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        match self.y.cmp(&other.y) {
            Ordering::Equal => Some(self.x.cmp(&other.x)),
            other => Some(other),
        }
    }
}

impl Unit {
    fn new(x: i32, y: i32, race: char, hp: i32, ap: i32) -> Unit {
        Unit { x, y, race, hp, ap }
    }

    fn adjacent_to(&self, other: &Unit) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() == 1
    }

    fn adjacent_space(&self, point: &Point) -> bool {
        (self.x - point.x).abs() + (self.y - point.y).abs() == 1
    }

    fn update_loc(&mut self, point: &Point) {
        self.x = point.x;
        self.y = point.y;
    }
    fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
        if self.hp <= 0 {
            self.race = 'D';
        }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        match self.y.cmp(&other.y) {
            Ordering::Equal => Some(self.x.cmp(&other.x)),
            other => Some(other),
        }
    }
}

fn pretty_print(field: &Vec<Vec<bool>>, units: &Vec<Unit>) {
    field.iter().enumerate().for_each(|pair| {
        let (y, row) = pair;
        let mut side_info = String::from("    ");
        print!(
            "{:?}",
            row.iter()
                .enumerate()
                .map(|ipair| {
                    let unit = units
                        .iter()
                        .find(|x| x.x == ipair.0 as i32 && x.y == y as i32);
                    match unit {
                        Some(u) => {
                            side_info.push(u.race.clone());
                            side_info.push_str(&u.hp.clone().to_string());
                            side_info.push_str(", ");
                            u.race
                        }
                        _ => if *ipair.1 {
                            '.'
                        } else {
                            '#'
                        },
                    }
                }).collect::<String>()
        );
        println!("{}", side_info);
    });
}

fn find_flawless_victory(filename: String) -> i32 {
    for i in 4..201i32 {
        let result = find_end_score(filename.clone(), i);
        if result.1 {
            return result.0;
        }
    }
    -1
}

fn find_end_score(filename: String, ap: i32) -> (i32, bool) {
    let (field, mut units) = parse_input(filename, ap);
    let mut round = 0;
    let elves_count = units.iter().filter(|x| x.race == 'E').count();

    loop {
        let turn_order = units.clone();
        let mut full_turn = true;

        for unit in turn_order.iter() {
            match units
                .iter()
                .find(|x| unit.x == x.x && unit.y == x.y && x.race == 'D')
            {
                Some(_) => continue,
                _ => (),
            }

            let targets: Vec<Unit> = units
                .iter()
                .filter(|x| unit.race != x.race && x.race != 'D')
                .cloned()
                .collect();
            let mut post_move_unit = unit.clone();

            if targets.len() == 0 {
                full_turn = false;
                continue;
            }

            if !targets.iter().any(|target| unit.adjacent_to(target)) {
                let mut visited: HashSet<Point> = HashSet::new();
                let mut todo: VecDeque<(Point, usize)> = VecDeque::new();
                let mut paths: HashMap<Point, Point> = HashMap::new();
                let mut valids: Vec<Point> = Vec::new();
                let mut best_step: Point;
                let mut max_depth = 1000;

                todo.push_back((Point::new(unit.x, unit.y), 0));

                while todo.len() > 0 {
                    let (current, depth) = todo.pop_front().unwrap();
                    if max_depth < depth {
                        continue;
                    }
                    if targets.iter().any(|target| target.adjacent_space(&current)) {
                        max_depth = depth;
                        valids.push(current.clone());
                    }
                    let children = vec![
                        Point::new(current.x, current.y - 1),
                        Point::new(current.x - 1, current.y),
                        Point::new(current.x + 1, current.y),
                        Point::new(current.x, current.y + 1),
                    ];
                    for child in children {
                        if visited.contains(&child)
                            || paths.contains_key(&child)
                            || child.is_blocked(&field, &units)
                        {
                            continue;
                        }
                        paths.insert(child.clone(), current.clone());
                        todo.push_back((child, depth + 1));
                    }
                    visited.insert(current.clone());
                }
                if valids.len() == 0 {
                    continue;
                }

                valids.sort_unstable();
                best_step = (*paths.get(&valids[0]).unwrap()).clone();
                let mut previous_step = best_step.clone();
                while paths.contains_key(&best_step) {
                    previous_step = best_step.clone();
                    best_step = (*paths.get(&best_step).unwrap()).clone();
                }
                if best_step != previous_step {
                    best_step = previous_step;
                } else {
                    best_step = valids[0].clone();
                }

                for loc_unit in units.iter_mut() {
                    if !(*loc_unit == *unit) {
                        continue;
                    }
                    (*loc_unit).update_loc(&best_step);
                    post_move_unit = *loc_unit;
                }
            }

            let mut best_target;
            {
                let mut valid_targets: Vec<Unit> = targets
                    .iter()
                    .filter(|target| post_move_unit.adjacent_to(target))
                    .cloned()
                    .collect();
                if valid_targets.len() == 0 {
                    continue;
                }
                valid_targets.sort_unstable();
                best_target = valid_targets.iter().cloned().min_by_key(|x| x.hp);
            }

            match best_target {
                Some(target) => units
                    .iter_mut()
                    .find(|x| **x == target)
                    .unwrap()
                    .take_damage(unit.ap),
                None => (),
            }
        }

        units = units.iter().filter(|x| x.race != 'D').cloned().collect();
        units.sort_unstable();
        let race = units[0].race.clone();
        if full_turn {
            round += 1;
        }
        if !units.iter().any(|unit| unit.race != race) {
            break;
        }
    }

    units = units.iter().filter(|x| x.race != 'D').cloned().collect();

    let tot_hp = units.iter().fold(0, |acc, x| acc + x.hp);

    (
        round * tot_hp,
        units[0].race == 'E' && elves_count == units.len(),
    )
}

fn parse_input(filename: String, ap: i32) -> (Vec<Vec<bool>>, Vec<Unit>) {
    let raw_data = utility::load_strings(filename);
    let mut field: Vec<Vec<bool>> = Vec::new();
    let mut units: Vec<Unit> = Vec::new();

    raw_data.iter().enumerate().for_each(|pair| {
        let (y, raw_row) = pair;
        field.push(Vec::new());
        raw_row.chars().enumerate().for_each(|col_pair| {
            let (x, val) = col_pair;
            match val {
                '#' => field[y].push(false),
                '.' => field[y].push(true),
                'E' => {
                    field[y].push(true);
                    units.push(Unit::new(x as i32, y as i32, 'E', 200, ap));
                }
                'G' => {
                    field[y].push(true);
                    units.push(Unit::new(x as i32, y as i32, 'G', 200, 3));
                }
                _ => panic!("New character?"),
            }
        })
    });

    (field, units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_do_custom_move() {
        let actual = find_end_score("./src/day15/custom.txt".to_string(), 3);
        assert_eq!(actual.0, 18036);
    }

    #[test]
    fn should_solve_the_sample_field() {
        let actual = find_end_score("./src/day15/test.txt".to_string(), 3);
        assert_eq!(actual.0, 27730);

        let actual = find_end_score("./src/day15/test2.txt".to_string(), 3);
        assert_eq!(actual.0, 36334);

        let actual = find_end_score("./src/day15/test3.txt".to_string(), 3);
        assert_eq!(actual.0, 39514);

        let actual = find_end_score("./src/day15/test4.txt".to_string(), 3);
        assert_eq!(actual.0, 27755);

        let actual = find_end_score("./src/day15/test5.txt".to_string(), 3);
        assert_eq!(actual.0, 28944);

        let actual = find_end_score("./src/day15/test6.txt".to_string(), 3);
        assert_eq!(actual.0, 18740);

        let actual = find_flawless_victory("./src/day15/test.txt".to_string());
        assert_eq!(actual, 4988);

        let actual = find_flawless_victory("./src/day15/test3.txt".to_string());
        assert_eq!(actual, 31284);

        let actual = find_flawless_victory("./src/day15/test4.txt".to_string());
        assert_eq!(actual, 3478);

        let actual = find_flawless_victory("./src/day15/test5.txt".to_string());
        assert_eq!(actual, 6474);

        let actual = find_flawless_victory("./src/day15/test6.txt".to_string());
        assert_eq!(actual, 1140);
    }

    #[test]
    fn should_read_initial_field() {
        let actual = parse_input("./src/day15/test.txt".to_string(), 3);
        actual.0.iter().for_each(|row| {
            println!(
                "{:?}",
                row.iter()
                    .map(|x| if *x { '.' } else { '#' })
                    .collect::<String>()
            );
        });
        assert_eq!(
            actual.0,
            [
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, true, false],
                [false, true, true, true, true, true, false],
                [false, true, false, true, false, true, false],
                [false, true, true, true, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, false, false]
            ]
        );
    }

}
