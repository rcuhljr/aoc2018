use super::utility;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;

pub fn solve_a() -> String {
    count_water("input17.txt".to_string()).0.to_string()
}

pub fn solve_b() -> String {
    count_water("input17.txt".to_string()).1.to_string()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn deeper(self) -> Point {
        Point::new(self.x, self.y + 1)
    }
    fn left(self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    fn right(self) -> Point {
        Point::new(self.x + 1, self.y)
    }
    fn higher(self) -> Point {
        Point::new(self.x, self.y - 1)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sample {
    before: [usize; 4],
    op_code: [usize; 4],
    after: [usize; 4],
}

fn count_water(filename: String) -> (usize, usize) {
    let (mut blockers, (min_y, max_y)) = parse_input(filename);
    let clay = blockers.len();
    let mut water: HashSet<Point> = HashSet::new();
    let mut sources: Vec<Point> = vec![Point::new(500, 0)];

    while sources.len() > 0 {
        let source = sources.pop().unwrap();
        let mut new_sources: Vec<Point> =
            count_source(&source, min_y, max_y, &mut blockers, &mut water);

        sources.append(&mut new_sources);
    }

    (water.len(), blockers.len() - clay)
}

fn count_source(
    source: &Point,
    min_y: usize,
    max_y: usize,
    blockers: &mut HashSet<Point>,
    water: &mut HashSet<Point>,
) -> Vec<Point> {
    let mut new_sources = vec![];

    let mut current = source.clone();

    loop {
        if current.y > max_y {
            break;
        }

        if !blockers.contains(&current.deeper()) && !water.contains(&current.deeper()) {
            if current.y >= min_y {
                water.insert(current);
            }
            current = current.deeper();
            continue;
        } else if !blockers.contains(&current.deeper()) {
            if current.y >= min_y {
                water.insert(current);
            }
            break;
        }

        let mut left_drain = false;
        let mut right_drain = false;
        let mut left = current.left();
        let mut right = current.right();
        let mut left_blocked = false;
        let mut right_blocked = false;

        while !left_drain && !left_blocked {
            if blockers.contains(&left) {
                left_blocked = true;
                left = left.right()
            } else if !blockers.contains(&left.deeper()) {
                left_drain = true;
            } else {
                left = left.left();
            }
        }
        while !right_drain && !right_blocked {
            if blockers.contains(&right) {
                right_blocked = true;
                right = right.left();
            } else if !blockers.contains(&right.deeper()) {
                right_drain = true;
            } else {
                right = right.right();
            }
        }

        if left_blocked && right_drain {
            for x in left.x..right.x {
                water.insert(Point::new(x, current.y));
            }
            new_sources.push(right);
            break;
        } else if right_blocked && left_drain {
            for x in left.x + 1..right.x + 1 {
                water.insert(Point::new(x, current.y));
            }
            new_sources.push(left);
            break;
        } else if right_drain && left_drain {
            for x in left.x + 1..right.x {
                water.insert(Point::new(x, current.y));
            }
            new_sources.push(left);
            new_sources.push(right);
            break;
        } else {
            for x in left.x..right.x + 1 {
                water.insert(Point::new(x, current.y));
                blockers.insert(Point::new(x, current.y));
            }
            current = current.higher();
        }
    }
    new_sources
}

fn parse_input(filename: String) -> (HashSet<Point>, (usize, usize)) {
    lazy_static! {
        static ref VERTICAL_REGEX: Regex = Regex::new(r"x=(\d+), y=(\d+)\.\.(\d+)").unwrap();
        static ref HORIZONTAL_REGEX: Regex = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)").unwrap();
    }
    let raw_data = utility::load_strings(filename);
    let mut field: HashSet<Point> = HashSet::new();
    let mut vert: Vec<(usize, usize, usize)> = Vec::new();
    let mut hori: Vec<(usize, usize, usize)> = Vec::new();
    let (mut min_y, mut max_y) = (1500, 0);

    for line in raw_data.iter() {
        if let Some(caps) = VERTICAL_REGEX.captures(line) {
            let val = (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            );
            vert.push(val);
            min_y = cmp::min(min_y, val.1);
            max_y = cmp::max(max_y, val.2);
            continue;
        }

        if let Some(caps) = HORIZONTAL_REGEX.captures(line) {
            let val = (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            );
            hori.push(val);
            min_y = cmp::min(min_y, val.0);
            max_y = cmp::max(max_y, val.0);
        }
    }

    for (x, ystart, ystop) in vert {
        for y in ystart..ystop + 1 {
            field.insert(Point::new(x, y));
        }
    }

    for (y, xstart, xstop) in hori {
        for x in xstart..xstop + 1 {
            field.insert(Point::new(x, y));
        }
    }

    (field, (min_y, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample_water_count() {
        let actual = count_water("./src/day17/test.txt".to_string());

        assert_eq!(actual.0, 57);
        assert_eq!(actual.1, 29);
    }

    #[test]
    fn should_solve_simple_bucket() {
        let mut blockers = parse_input("./src/day17/test.txt".to_string()).0;
        let mut water = HashSet::new();

        let actual = count_source(&Point::new(500, 0), 1, 13, &mut blockers, &mut water);

        assert_eq!(water.len(), 18);
        assert_eq!(actual, vec![Point::new(502, 2)]);
    }

    #[test]
    fn should_read_initial_input16() {
        let actual = parse_input("./src/day17/test.txt".to_string());
        assert_eq!(actual.1, (1, 13));
        assert_eq!(actual.0.len(), 34);
    }

}
