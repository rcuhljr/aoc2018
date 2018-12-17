use std::fs::File;
use std::io::prelude::*;
use std::str::Lines;

pub fn load_numbers(filename: String) -> Vec<i32> {
    return load_strings(filename)
        .iter()
        .map(|val| val.parse().unwrap())
        .collect();
}

pub fn load_strings(filename: String) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let v: Lines = contents.lines();

    return v.map(|val| val.to_string()).collect();
}

// #[macro_export]
// macro_rules! debugln {
//     ($fmt:expr, $log:expr) => (if $log {println!($fmt)});
//     ($fmt:expr, $log:expr, $($arg:tt)*) => (if $log {println!($fmt,$($arg)*)});
// }

// #[macro_export]
// macro_rules! debug {
//     ($fmt:expr) => (if true {println!($fmt)});
//     ($fmt:expr, $($arg:tt)*) => (if true {println!($fmt,$($arg)*)});
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let v = vec![1, 2, -1];

        assert_eq!(load_numbers("./src/utility/test.txt".to_string()), v);
    }

    #[test]
    fn read_string_file() {
        let v = vec![String::from("ABC"), String::from("DEF")];

        assert_eq!(
            load_strings("./src/utility/test_strings.txt".to_string()),
            v
        );
    }

}
