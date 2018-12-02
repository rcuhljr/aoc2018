use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Lines;

pub fn load_adjustments(filename: String) -> Vec<i32> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let v: Lines = contents.lines();

    return v.map(|val| val.parse().unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let v = vec![1, 2, -1];

        assert!(load_adjustments("./src/utility/test.txt".to_string()) == v);
    }

}
