use super::utility;
use std::cmp;

pub fn solve_a() -> String {
    collapse_polymers("input5.txt".to_string())
        .len()
        .to_string()
}

pub fn solve_b() -> String {
    find_shortest("input5.txt".to_string()).to_string()
}

pub fn collapse_polymers(filename: String) -> String {
    collapse_polymers_string(
        utility::load_strings(filename)
            .iter()
            .next()
            .unwrap()
            .to_string(),
    )
}
pub fn collapse_polymers_string(polystring: String) -> String {
    let mut polymers: Vec<char> = polystring.chars().collect();

    let mut delete_at: i32 = -1;
    let mut index: i32 = 0;
    loop {
        let mut last_char = ' ';
        if delete_at >= 0 {
            polymers.remove(delete_at as usize);
            polymers.remove(delete_at as usize);
            delete_at = -1
        }
        for char_index in index..(polymers.len() as i32) {
            let item = polymers.get(char_index as usize).unwrap();
            if last_char.to_ascii_uppercase() == item.to_ascii_uppercase() && last_char != *item {
                delete_at = index - 1;
                index = cmp::max(0, index - 2);
                break;
            }
            last_char = *item;
            index += 1;
        }
        if delete_at == -1 {
            break;
        }
    }

    polymers.iter().map(|c| *c).collect()
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn find_shortest(filename: String) -> usize {
    let base_string = utility::load_strings(filename)
        .iter()
        .next()
        .unwrap()
        .to_string();
    let mut results: Vec<usize> = Vec::new();

    for item in ASCII_LOWER.iter() {
        let item2 = item.to_ascii_uppercase();
        let mod_string = base_string
            .chars()
            .filter(|&c| !(c == *item || c == item2))
            .collect();
        let result = collapse_polymers_string(mod_string);
        results.push(result.len());
    }

    *results.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn should_solve_sample5a() {
        let actual: String = collapse_polymers("./src/day5/test.txt".to_string());
        assert_eq!(actual.len(), 10);
        assert_eq!(actual, "dabCBAcaDA");
    }

    #[test]
    fn should_solve_sample5b() {
        let actual = find_shortest("./src/day5/test.txt".to_string());
        assert_eq!(actual, 4);
    }
    // Uncomment when speed fix is in place.
    // #[bench]
    // fn bench_a(b: &mut Bencher) {
    //     b.iter(|| solve_a());
    // }

    // #[bench]
    // fn bench_b(b: &mut Bencher) {
    //     b.iter(|| solve_b());
    // }
}
