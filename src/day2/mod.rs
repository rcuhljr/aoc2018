use super::utility;
use std::collections::HashMap;

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
