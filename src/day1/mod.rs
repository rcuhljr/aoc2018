use super::utility;
use std::collections::HashMap;

fn total_adjustments(adjusts: Vec<i32>) -> i32 {
    return adjusts.iter().fold(0, |acc, x| acc + x);
}

fn stops_twice(adjusts: Vec<i32>) -> i32 {
    let mut stops = HashMap::new();
    let mut current = 0;
    let mut looped_adjusts = adjusts.iter().cycle();

    loop {
        if stops.contains_key(&current) {
            return current;
        }
        stops.insert(current.clone(), true);
        current += looped_adjusts.next().unwrap();
    }
}

pub fn solve_a() -> String {
    return total_adjustments(utility::load_numbers("input1a.txt".to_string())).to_string();
}

pub fn solve_b() -> String {
    return stops_twice(utility::load_numbers("input1a.txt".to_string())).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all_positives_input() {
        let v = vec![1, 1, 1];

        assert_eq!(total_adjustments(v), 3);
    }

    #[test]
    fn mixed_inputs() {
        let v = vec![1, 1, -2];

        assert_eq!(total_adjustments(v), 0);
    }

    #[test]
    fn reaches_zero_simple() {
        let v = vec![1, -1];

        assert_eq!(stops_twice(v), 0);
    }

    #[test]
    fn reaches_ten() {
        let v = vec![3, 3, 4, -2, -4];

        assert_eq!(stops_twice(v), 10);
    }

    #[bench]
    fn bench_a(b: &mut Bencher) {
        b.iter(|| solve_a());
    }

    #[bench]
    fn bench_b(b: &mut Bencher) {
        b.iter(|| solve_b());
    }
}
