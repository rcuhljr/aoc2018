use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_a() -> String {
    let depth = 4002;
    let target = Point::new(5, 746);
    let map = build_map(depth, target);
    let actual = count_risk_in_rect(Point::new(0, 0), target, &map);
    actual.to_string()
}

pub fn solve_b() -> String {
    let depth = 4002;
    let target = Point::new(5, 746);
    let map = build_map(depth, target);
    let actual = find_shortest_path(&map, target);
    actual.to_string()
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

    fn left(self) -> Point {
        if self.x == 0 {
            Point::new(100000, 100000)
        } else {
            Point::new(self.x - 1, self.y)
        }
    }
    fn right(self) -> Point {
        Point::new(self.x + 1, self.y)
    }
    fn up(self) -> Point {
        if self.y == 0 {
            Point::new(100000, 100000)
        } else {
            Point::new(self.x, self.y - 1)
        }
    }
    fn down(self) -> Point {
        Point::new(self.x, self.y + 1)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Region {
    terrain: usize,
    erosion: usize,
    loc: Point,
}

impl Region {
    fn new(loc: Point, depth: usize, target: Point, map: &HashMap<Point, Region>) -> Region {
        let erosion;
        if (loc.x == 0 && loc.y == 0) || (loc == target) {
            erosion = depth % 20183;
        } else if loc.x == 0 {
            erosion = ((loc.y * 48271) + depth) % 20183;
        } else if loc.y == 0 {
            erosion = ((loc.x * 16807) + depth) % 20183;
        } else {
            erosion = ((map.get(&loc.left()).unwrap().erosion
                * map.get(&loc.up()).unwrap().erosion)
                + depth)
                % 20183;
        }
        let terrain = erosion % 3;
        Region {
            terrain,
            erosion,
            loc,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Inst {
    op_code: String,
    vals: [usize; 3],
}

fn build_map(depth: usize, target: Point) -> HashMap<Point, Region> {
    let mut map = HashMap::new();

    for x in 0..target.x + 50 {
        for y in 0..target.y + 30 {
            let loc = Point::new(x, y);
            let region = Region::new(loc, depth, target, &map);
            map.insert(loc, region);
        }
    }
    map
}

fn count_risk_in_rect(
    upper_left: Point,
    lower_right: Point,
    map: &HashMap<Point, Region>,
) -> usize {
    let mut risk = 0;
    for y in upper_left.y..lower_right.y + 1 {
        for x in upper_left.x..lower_right.x + 1 {
            let loc = Point::new(x, y);
            risk += map.get(&loc).unwrap().terrain;
        }
    }
    risk
}

fn find_shortest_path(map: &HashMap<Point, Region>, target: Point) -> usize {
    let mut visited: HashSet<(Point, usize)> = HashSet::new();
    let mut todo: Vec<(Point, usize, usize)> = vec![];

    todo.push((Point::new(0, 0), 0, 1));

    while todo.len() > 0 {
        let (current, time, tool) = todo.pop().unwrap();
        if visited.contains(&(current, tool)) {
            continue;
        }

        if current == target {
            if tool == 1 {
                return time;
            } else {
                return time + 7;
            }
        }

        let mut children = vec![
            current.down(),
            current.right(),
            current.left(),
            current.up(),
        ];

        children = children
            .iter()
            .filter(|x| map.contains_key(*x))
            .cloned()
            .collect();

        if children.len() == 0 {
            continue;
        }

        let terrain = map.get(&current).unwrap().terrain;
        for child in children {
            let new_terrain = map.get(&child).unwrap().terrain;
            let new_todo;
            if terrain == new_terrain {
                new_todo = (child, time + 1, tool);
            } else if terrain == 0 && new_terrain == 1 {
                if tool == 2 {
                    new_todo = (child, time + 1, tool);
                } else {
                    new_todo = (child, time + 8, 2);
                }
            } else if terrain == 0 && new_terrain == 2 {
                if tool == 1 {
                    new_todo = (child, time + 1, tool);
                } else {
                    new_todo = (child, time + 8, 1);
                }
            } else if terrain == 1 && new_terrain == 0 {
                if tool == 2 {
                    new_todo = (child, time + 1, tool);
                } else {
                    new_todo = (child, time + 8, 2);
                }
            } else if terrain == 1 && new_terrain == 2 {
                if tool == 0 {
                    new_todo = (child, time + 1, tool);
                } else {
                    new_todo = (child, time + 8, 0);
                }
            } else if terrain == 2 && new_terrain == 0 {
                if tool == 1 {
                    new_todo = (child, time + 1, tool);
                } else {
                    new_todo = (child, time + 8, 1);
                }
            } else {
                if tool == 0 {
                    new_todo = (child, time + 1, tool);
                } else {
                    new_todo = (child, time + 8, 0);
                }
            }
            todo.push(new_todo);
        }
        visited.insert((current.clone(), tool));
        todo.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample22() {
        let depth = 510;
        let target = Point::new(10, 10);
        let map = build_map(depth, target);
        let actual = count_risk_in_rect(Point::new(0, 0), target, &map);

        assert_eq!(actual, 114);
    }

    #[test]
    fn should_solve_sample_2() {
        let depth = 510;
        let target = Point::new(10, 10);
        let map = build_map(depth, target);
        let actual = find_shortest_path(&map, target);
        assert_eq!(actual, 45);
    }
}
