use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use utility;

pub fn solve_a() -> String {
    let data = parse_star_data("input25.txt".to_string());
    count_constellations(&data).to_string()
}

pub fn solve_b() -> String {
    "Redacted To Prevent Spoilers!".to_string()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32, t: i32) -> Point {
        Point { x, y, z, t }
    }
}

fn parse_star_data(filename: String) -> Vec<Point> {
    lazy_static! {
        static ref POINT_REGEX: Regex = Regex::new(r"\s*(.*),(.*),(.*),(.*)").unwrap();
    }
    let raw_fields = utility::load_strings(filename.to_string());
    raw_fields
        .iter()
        .map(|raw| {
            let caps = POINT_REGEX.captures(raw).unwrap();
            Point::new(
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            )
        }).collect()
}

fn count_constellations(stars: &Vec<Point>) -> usize {
    let mut constellations: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut const_map: HashMap<Point, Point> = HashMap::new();

    for star in stars.iter() {
        let mut touching_stars = HashSet::new();
        for x in -3..4i32 {
            for y in (-3 + x.abs())..(4 - x.abs()) {
                for z in (-3 + x.abs() + y.abs())..(4 - x.abs() - y.abs()) {
                    for t in (-3 + x.abs() + y.abs() + z.abs())..(4 - x.abs() - y.abs() - z.abs()) {
                        let potential = Point::new(star.x + x, star.y + y, star.z + z, star.t + t);
                        if const_map.contains_key(&potential) {
                            touching_stars.insert(const_map.get(&potential).unwrap().clone());
                        }
                    }
                }
            }
        }
        if touching_stars.len() > 0 {
            let mut new_key = star.clone();
            let mut all_joined_stars: Vec<Point> = touching_stars
                .iter()
                .flat_map(|x| constellations.get(x).unwrap().iter())
                .cloned()
                .collect();
            touching_stars.iter().for_each(|x| {
                constellations.remove(x);
            });

            all_joined_stars.push(new_key);

            all_joined_stars.iter().for_each(|x| {
                const_map.insert(*x, new_key);
            });
            constellations.insert(new_key, all_joined_stars);
        } else {
            const_map.insert(*star, *star);
            constellations.insert(*star, vec![*star]);
        }
    }

    constellations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_simple_constellations() {
        let data = parse_star_data("./src/day25/test.txt".to_string());
        let actual = count_constellations(&data);

        assert_eq!(actual, 2);

        let data = parse_star_data("./src/day25/test2.txt".to_string());
        let actual = count_constellations(&data);

        assert_eq!(actual, 4);
    }

    #[test]
    fn should_read_star_data() {
        let actual = parse_star_data("./src/day25/test.txt".to_string());

        assert_eq!(
            actual,
            [
                Point {
                    x: 0,
                    y: 0,
                    z: 0,
                    t: 0
                },
                Point {
                    x: 3,
                    y: 0,
                    z: 0,
                    t: 0
                },
                Point {
                    x: 0,
                    y: 3,
                    z: 0,
                    t: 0
                },
                Point {
                    x: 0,
                    y: 0,
                    z: 3,
                    t: 0
                },
                Point {
                    x: 0,
                    y: 0,
                    z: 0,
                    t: 3
                },
                Point {
                    x: 0,
                    y: 0,
                    z: 0,
                    t: 6
                },
                Point {
                    x: 9,
                    y: 0,
                    z: 0,
                    t: 0
                },
                Point {
                    x: 12,
                    y: 0,
                    z: 0,
                    t: 0
                }
            ]
        );
    }

}
