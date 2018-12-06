use super::utility;
use std::collections::HashMap;

pub fn solve_a() -> String {
    find_largest_area("input6.txt").to_string()
}

pub fn solve_b() -> String {
    find_safe_area("input6.txt", 10000).to_string()
}

fn parse_data(data: Vec<String>, goals: &mut HashMap<(i32, i32), i32>) -> i32 {
    let mut max_dim: i32 = 0;
    let mut index: i32 = 0;
    data.iter().for_each(|raw| {
        let v: Vec<i32> = raw
            .split(", ")
            .map(|val| val.trim().parse().unwrap())
            .collect();
        goals.insert((v[0], v[1]), index);
        if v[0] > max_dim {
            max_dim = v[0]
        }
        if v[1] > max_dim {
            max_dim = v[1]
        }
        index += 1;
    });

    max_dim + 1
}

fn find_safe_area(filename: &str, cutoff: i32) -> i32 {
    let mut goals: HashMap<(i32, i32), i32> = HashMap::new();
    let size: i32 = parse_data(utility::load_strings(filename.to_string()), &mut goals);
    let mut safe_areas = 0;

    for index in 0..size * size {
        let loc_x: i32 = (index as i32) % size;
        let loc_y: i32 = (index as i32) / size;
        let total_dist = goals.keys().fold(0, |acc, coords| {
            acc + (coords.0 - loc_x).abs() + (coords.1 - loc_y).abs()
        });
        if total_dist < cutoff {
            safe_areas += 1;
        }
    }
    safe_areas
}

fn find_largest_area(filename: &str) -> i32 {
    let mut infinite = HashMap::new();
    let mut goals: HashMap<(i32, i32), i32> = HashMap::new();
    let mut result = HashMap::new();
    let size: i32 = parse_data(utility::load_strings(filename.to_string()), &mut goals);

    for index in 0..(size * size) {
        let loc_x: i32 = (index as i32) % size;
        let loc_y: i32 = (index as i32) / size;
        let mut distances: Vec<(i32, i32)> = goals
            .iter()
            .map(|(coords, id)| {
                let dist = (coords.0 - loc_x).abs() + (coords.1 - loc_y).abs();
                (dist, *id)
            }).collect();
        distances.sort_by_key(|pair| pair.0);
        if distances[0].0 != distances[1].0 {
            if loc_x == 0 || loc_x == size - 1 || loc_y == 0 || loc_y == size - 1 {
                infinite.insert(distances[0].1.clone(), true);
            }
            let counter = result.entry(distances[0].1).or_insert(0);
            *counter += 1;
        }
    }

    goals
        .values()
        .filter_map(|x| {
            if infinite.contains_key(x) {
                None
            } else {
                Some(*x)
            }
        }).map(|x| (x, *result.get(&x).unwrap()))
        .max_by_key(|x| x.1)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample6a() {
        let actual: i32 = find_largest_area("./src/day6/test.txt");
        assert_eq!(actual, 17);
    }

    #[test]
    fn should_solve_sample6b() {
        let actual: i32 = find_safe_area("./src/day6/test.txt", 32);
        assert_eq!(actual, 16);
    }
}
