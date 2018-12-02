use super::utility;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Lines;

pub fn solve_a() -> String {
    return "".to_string();
}

pub fn solve_b() -> String {
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn read_file() {
        let v = vec![1, 2, -1];

        assert!(utility::load_adjustments("./src/utility/test.txt".to_string()) == v);
    }
}
