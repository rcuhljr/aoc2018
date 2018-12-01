use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Lines;

fn load_adjustments(filename: String) -> Vec<i32> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let v: Lines = contents.lines();

    return v.map(|val| val.parse().unwrap()).collect();
}

fn total_adjustments(adjusts: Vec<i32>) -> i32 {
    return adjusts.iter().fold(0, |acc, x| acc + x);
}

fn stops_twice(adjusts: Vec<i32>) -> i32 {
    let mut stops = HashMap::new();
    let mut current = 0;
    let mut index = 0;

    loop {
        if stops.contains_key(&current) {
            return current;
        }
        stops.insert(current.clone(), true);
        current += adjusts[index % adjusts.len()];
        index += 1;
    }
}

pub fn solve_a() -> String {
    return total_adjustments(load_adjustments("input1a.txt".to_string())).to_string();
}

pub fn solve_b() -> String {
    return stops_twice(load_adjustments("input1a.txt".to_string())).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn all_positives_input() {
        let v = vec![1, 1, 1];

        assert!(total_adjustments(v) == 3);
    }

    #[test]
    fn read_file() {
        let v = vec![1, 2, -1];

        assert!(load_adjustments("./src/day1/test.txt".to_string()) == v);
    }

    #[test]
    fn mixed_inputs() {
        let v = vec![1, 1, -2];

        assert!(total_adjustments(v) == 0);
    }

    #[test]
    fn reaches_zero_simple() {
        let v = vec![1, -1];

        assert!(stops_twice(v) == 0);
    }

    #[test]
    fn reaches_ten() {
        let v = vec![3, 3, 4, -2, -4];

        assert!(stops_twice(v) == 10);
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
