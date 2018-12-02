use super::utility;
use std::collections::HashMap;

fn find_checksum(samples: Vec<String>) -> i32 {
    let mut doubles = 0;
    let mut triples = 0;
    samples.iter().for_each(|sample| {
        let mut counts = HashMap::new();
        sample.chars().for_each(|single| {
            let counter = counts.entry(single).or_insert(0);
            *counter += 1;
        });
        if counts.values().any(|&x| x == 2) {
            doubles += 1;
        }
        if counts.values().any(|&x| x == 3) {
            triples += 1;
        }
    });
    return doubles * triples;
}

fn find_near_match(samples: Vec<String>) -> String {
    for index in 0..samples.len() - 1 {
        for inner_index in index + 1..samples.len() {
            let result = check_match(&samples[index], &samples[inner_index]);
            if result != "".to_string() {
                return result;
            }
        }
    }
    return "".to_string();
}

fn check_match(left: &String, right: &String) -> String {
    let mut common = String::new();
    let mut missmatch = false;
    let mut pairs = left.chars().zip(right.chars());

    return match pairs.try_for_each(|(chl, chr)| {
        if chl == chr {
            common.push(chl)
        } else {
            if missmatch {
                return Err(());
            } else {
                missmatch = true;
            }
        }
        return Ok(());
    }) {
        Ok(()) => common,
        Err(()) => "".to_string(),
    };
}

pub fn solve_a() -> String {
    return find_checksum(utility::load_strings("input2a.txt".to_string())).to_string();
}

pub fn solve_b() -> String {
    return find_near_match(utility::load_strings("input2a.txt".to_string())).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn find_checksum_sample() {
        let v = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];

        assert_eq!(find_checksum(v), 12);
    }

    #[test]
    fn check_match_sample() {
        let actual = check_match(&String::from("fguij"), &String::from("fghij"));
        assert_eq!(actual, String::from("fgij"));
    }

    #[test]
    fn find_near_match_sample() {
        let v = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
        ];
        assert_eq!(find_near_match(v), String::from("fgij"));
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
