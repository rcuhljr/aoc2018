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
    loc: Point,
    race: char,
    hp: i32,
    ap: i32,
    turn: bool,
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

    fn is_blocked(&self, field: &Vec<Vec<bool>>, units_map: &HashMap<Point, Unit>) -> bool {
        !field[self.y as usize][self.x as usize] || units_map.contains_key(&self)
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

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            other => other,
        }
    }
}

impl Unit {
    fn new(loc: Point, race: char, hp: i32, ap: i32, turn: bool) -> Unit {
        Unit {
            loc,
            race,
            hp,
            ap,
            turn,
        }
    }

    fn adjacent_to(&self, other: &Unit) -> bool {
        (self.loc.x - other.loc.x).abs() + (self.loc.y - other.loc.y).abs() == 1
    }

    fn adjacent_space(&self, point: &Point) -> bool {
        (self.loc.x - point.x).abs() + (self.loc.y - point.y).abs() == 1
    }

    fn update_loc(&mut self, point: &Point) {
        self.loc.x = point.x;
        self.loc.y = point.y;
    }
    fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
        if self.hp <= 0 {
            self.race = 'D';
        }
    }
    fn take_turn(&mut self) {
        self.turn = true;
    }

    fn reset_turn(&mut self) {
        self.turn = false;
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        Some(self.loc.cmp(&other.loc))
    }
}

#[allow(dead_code)]
fn pretty_print(field: &Vec<Vec<bool>>, units_map: &HashMap<Point, Unit>) {
    field.iter().enumerate().for_each(|pair| {
        let (y, row) = pair;
        let mut side_info = String::from("    ");
        print!(
            "{}",
            row.iter()
                .enumerate()
                .map(|ipair| {
                    let unit = units_map.get(&Point::new(ipair.0 as i32, y as i32));
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

fn move_bfs_reading_order(
    unit: &mut Unit,
    targets: &Vec<Unit>,
    field: &Vec<Vec<bool>>,
    unit_map: &mut HashMap<Point, Unit>,
) -> Option<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut todo: VecDeque<(Point, usize)> = VecDeque::new();
    let mut paths: HashMap<Point, Point> = HashMap::new();
    let mut valids: Vec<Point> = Vec::new();
    let mut best_step: Point;
    let mut max_depth = 1000;

    todo.push_back((unit.loc, 0));

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
                || child.is_blocked(&field, &unit_map)
            {
                continue;
            }
            paths.insert(child.clone(), current.clone());
            todo.push_back((child, depth + 1));
        }
        visited.insert(current.clone());
    }
    if valids.len() == 0 {
        return None;
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

    let old_loc = unit.loc.clone();
    let mut moving_unit = unit_map.remove(&old_loc).unwrap();
    moving_unit.update_loc(&best_step);
    unit_map.insert(moving_unit.loc, moving_unit);
    unit.update_loc(&best_step);
    Some(best_step)
}

fn find_end_score(filename: String, ap: i32) -> (i32, bool) {
    let (field, units) = parse_input(filename, ap);
    let mut round = 0;
    let elves_count = units.iter().filter(|x| x.race == 'E').count();
    let mut winning_race: char;
    let mut unit_map: HashMap<Point, Unit> = HashMap::new();
    for unit in units.iter() {
        unit_map.insert(unit.loc.clone(), unit.clone());
    }

    loop {
        // println!("round: {}", round);
        // pretty_print(&field, &unit_map);

        unit_map.iter_mut().for_each(|pair| pair.1.reset_turn());
        let mut full_turn = true;

        while unit_map.values().any(|x| !x.turn) {
            let mut turn_order: Vec<Unit> = Vec::new();
            for val in unit_map.values() {
                if !val.turn {
                    turn_order.push(val.clone());
                }
            }
            turn_order.sort_unstable();
            let mut unit = turn_order[0];
            unit_map.get_mut(&unit.loc).unwrap().take_turn();

            let targets: Vec<Unit> = unit_map
                .values()
                .filter(|x| unit.race != x.race && x.race != 'D')
                .cloned()
                .collect();

            if targets.len() == 0 {
                full_turn = false;
                continue;
            }
            if !targets.iter().any(|target| unit.adjacent_to(target)) {
                match move_bfs_reading_order(&mut unit, &targets, &field, &mut unit_map) {
                    Some(new_loc) => unit = *unit_map.get_mut(&new_loc).unwrap(),
                    None => continue,
                }
            }

            let mut valid_targets: Vec<Unit> = targets
                .iter()
                .filter(|target| unit.adjacent_to(target))
                .cloned()
                .collect();
            if valid_targets.len() == 0 {
                continue;
            }
            valid_targets.sort_unstable();
            let best_target = valid_targets
                .iter()
                .cloned()
                .min_by_key(|x| x.hp)
                .unwrap()
                .loc;
            unit_map.get_mut(&best_target).unwrap().take_damage(unit.ap);
            if unit_map.get(&best_target).unwrap().race == 'D' {
                unit_map.remove(&best_target);
            }
        }

        let mut unit_iter = unit_map.values();
        winning_race = unit_iter.next().unwrap().race.clone();
        if full_turn {
            round += 1;
        }
        if !unit_iter.any(|unit| unit.race != winning_race) {
            break;
        }
    }

    // println!("round: {}", round);
    // pretty_print(&field, &unit_map);

    let tot_hp = unit_map.values().fold(0, |acc, x| acc + x.hp);
    (
        round * tot_hp,
        winning_race == 'E' && elves_count == unit_map.len(),
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
                    units.push(Unit::new(
                        Point::new(x as i32, y as i32),
                        'E',
                        200,
                        ap,
                        false,
                    ));
                }
                'G' => {
                    field[y].push(true);
                    units.push(Unit::new(
                        Point::new(x as i32, y as i32),
                        'G',
                        200,
                        3,
                        false,
                    ));
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
