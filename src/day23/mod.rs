use super::utility;
use regex::Regex;

pub fn solve_a() -> String {
    let bots = parse_input("input23.txt".to_string());
    count_bots_in_range(&bots).to_string()
}

pub fn solve_b() -> String {
    let bots = parse_input("input23.txt".to_string());
    let hrect: HRectangle = build_bound_box(&bots);
    find_most_in_range_point_dist(&bots, hrect).to_string()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct HRectangle {
    min: Point,
    max: Point,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Bot {
    loc: Point,
    r: i64,
}

impl Bot {
    fn new(x: i64, y: i64, z: i64, r: i64) -> Bot {
        let loc = Point::new(x, y, z);
        Bot { loc, r }
    }
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

impl HRectangle {
    fn new(min_x: i64, min_y: i64, min_z: i64, max_x: i64, max_y: i64, max_z: i64) -> HRectangle {
        let min = Point::new(min_x, min_y, min_z);
        let max = Point::new(max_x, max_y, max_z);
        HRectangle { min, max }
    }
    fn split(&self) -> (HRectangle, HRectangle) {
        let min = self.min;
        let max = self.max;
        let gapx = max.x - min.x;
        let gapy = max.y - min.y;
        let gapz = max.z - min.z;
        if gapx >= gapy && gapx >= gapz {
            (
                HRectangle::new(min.x, min.y, min.z, min.x + (gapx / 2), max.y, max.z),
                HRectangle::new(min.x + gapx / 2 + 1, min.y, min.z, max.x, max.y, max.z),
            )
        } else if gapy >= gapx && gapy >= gapz {
            (
                HRectangle::new(min.x, min.y, min.z, max.x, min.y + gapy / 2, max.z),
                HRectangle::new(min.x, min.y + gapy / 2 + 1, min.z, max.x, max.y, max.z),
            )
        } else {
            (
                HRectangle::new(min.x, min.y, min.z, max.x, max.y, min.z + gapz / 2),
                HRectangle::new(min.x, min.y, min.z + gapz / 2 + 1, max.x, max.y, max.z),
            )
        }
    }

    fn is_point(&self) -> bool {
        self.min == self.max
    }
}

fn build_bound_box(bots: &Vec<Bot>) -> HRectangle {
    let (mut min_x, mut min_y, mut min_z, mut max_x, mut max_y, mut max_z) =
        (10000000, 10000000, 10000000, 0, 0, 0);

    for bot in bots.iter() {
        if bot.loc.x < min_x {
            min_x = bot.loc.x
        }
        if bot.loc.x > max_x {
            max_x = bot.loc.x
        }
        if bot.loc.y < min_y {
            min_y = bot.loc.y
        }
        if bot.loc.y > max_y {
            max_y = bot.loc.y
        }
        if bot.loc.z < min_z {
            min_z = bot.loc.z
        }
        if bot.loc.z > max_z {
            max_z = bot.loc.z
        }
    }
    HRectangle::new(min_x, min_y, min_z, max_x, max_y, max_z)
}

fn parse_input(filename: String) -> Vec<Bot> {
    lazy_static! {
        static ref BOT_REGEX: Regex = Regex::new(r"^pos=<(.+),(.+),(.+)>, r=(.+)").unwrap();
    }
    let raw_data = utility::load_strings(filename);
    let data_pass = raw_data.iter();
    let mut bots: Vec<Bot> = Vec::new();

    for line in data_pass {
        if let Some(caps) = BOT_REGEX.captures(line) {
            bots.push(Bot::new(
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ));
        }
    }

    bots
}

fn count_bots_in_range(bots: &Vec<Bot>) -> usize {
    let largest = bots.iter().max_by_key(|bot| bot.r).unwrap();
    bots.iter()
        .filter(|bot| {
            ((largest.loc.x - bot.loc.x).abs()
                + (largest.loc.y - bot.loc.y).abs()
                + (largest.loc.z - bot.loc.z).abs())
                <= largest.r
        }).count()
}

fn find_most_in_range_point_dist(bots: &Vec<Bot>, hrect: HRectangle) -> i64 {
    let mut best_options = vec![hrect];
    loop {
        let mut new_options = vec![];
        let mut best_count = 0;
        for option in best_options.iter() {
            let count = bots
                .iter()
                .filter(|x| does_bot_reach_box(x, &option))
                .count();
            if count > best_count {
                best_count = count;
            }
            new_options.push((option.clone(), count));
        }

        if best_options[0].is_point() {
            return new_options
                .iter()
                .filter(|x| x.1 == best_count)
                .map(|hr| hr.0.min.x.abs() + hr.0.min.y.abs() + hr.0.min.z.abs())
                .min()
                .unwrap();
        }

        best_options = new_options
            .iter()
            .filter(|x| x.1 == best_count)
            .map(|x| x.0)
            .collect();

        let mut split_options = vec![];
        for option in best_options.iter() {
            let (left, right) = option.split();
            split_options.push(left);
            split_options.push(right);
        }
        best_options = split_options;
    }
}

fn does_bot_reach_box(bot: &Bot, hrect: &HRectangle) -> bool {
    //collapse z index
    let modr;
    if bot.loc.z < hrect.min.z {
        modr = bot.r - (hrect.min.z - bot.loc.z);
    } else if bot.loc.z > hrect.max.z {
        modr = bot.r - (bot.loc.z - hrect.max.z);
    } else {
        modr = bot.r;
    }
    if modr < 0 {
        return false;
    }
    let modx;
    if bot.loc.x < hrect.min.x {
        modx = hrect.min.x;
    } else if bot.loc.x > hrect.max.x {
        modx = hrect.max.x;
    } else {
        modx = bot.loc.x;
    }
    let mody;
    if bot.loc.y < hrect.min.y {
        mody = hrect.min.y;
    } else if bot.loc.y > hrect.max.y {
        mody = hrect.max.y;
    } else {
        mody = bot.loc.y;
    }

    let man_dist: i64 = (bot.loc.x - modx).abs() + (bot.loc.y - mody).abs();

    man_dist <= modr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_small_sample23() {
        let bots = parse_input("./src/day23/test.txt".to_string());
        let (mut min_x, mut min_y, mut min_z, mut max_x, mut max_y, mut max_z) =
            (10000000, 10000000, 10000000, 0, 0, 0);

        for bot in bots.iter() {
            if bot.loc.x < min_x {
                min_x = bot.loc.x
            }
            if bot.loc.x > max_x {
                max_x = bot.loc.x
            }
            if bot.loc.y < min_y {
                min_y = bot.loc.y
            }
            if bot.loc.y > max_y {
                max_y = bot.loc.y
            }
            if bot.loc.z < min_z {
                min_z = bot.loc.z
            }
            if bot.loc.z > max_z {
                max_z = bot.loc.z
            }
        }

        let hrect: HRectangle = HRectangle::new(min_x, min_y, min_z, max_x, max_y, max_z);

        let actual = find_most_in_range_point_dist(&bots, hrect);
        assert_eq!(actual, 36);
    }

    #[test]
    fn should_solve_bigger_sample23() {
        let bots = parse_input("./src/day23/test2.txt".to_string());
        let hrect: HRectangle = build_bound_box(&bots);

        let actual = find_most_in_range_point_dist(&bots, hrect);
        assert_eq!(actual, 1);
    }

    #[test]
    fn should_check_3d_intersection() {
        let bots = parse_input("./src/day23/test.txt".to_string());
        let hrect: HRectangle = HRectangle::new(12, 12, 12, 12, 12, 12);
        let mut results = vec![];
        for bot in bots.iter() {
            results.push(does_bot_reach_box(bot, &hrect));
        }
        assert_eq!(results, [true, true, true, true, true, false]);
    }

    #[test]
    fn should_find_densest_area() {
        let bots = parse_input("./src/day23/test.txt".to_string());
        let hrect: HRectangle = build_bound_box(&bots);

        assert_eq!(
            hrect,
            HRectangle {
                min: Point {
                    x: 10,
                    y: 10,
                    z: 10
                },
                max: Point {
                    x: 50,
                    y: 50,
                    z: 50
                }
            }
        );
    }

    #[test]
    fn should_find_bots_in_range() {
        let bots = parse_input("./src/day23/test2.txt".to_string());
        let actual = count_bots_in_range(&bots);
        assert_eq!(actual, 7);
    }
}
