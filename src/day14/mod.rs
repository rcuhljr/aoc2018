pub fn solve_a() -> String {
    find_next_scores("37".to_string(), 360781, 10)
}

pub fn solve_b() -> String {
    find_part_2("37".to_string(), 360781).to_string()
}

fn parse_input(raw_input: String) -> Vec<usize> {
    raw_input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
}

fn find_part_2(raw_input: String, start: usize) -> usize {
    let mut recipes = parse_input(raw_input.clone());
    let mut elves_loc: Vec<usize> = vec![0, 1];
    let target_arr = parse_input(start.to_string());
    let last_digit = target_arr[target_arr.len() - 1];

    loop {
        let new_score: usize = elves_loc.iter().map(|x| recipes[*x]).sum();
        let new_recipes = parse_input(new_score.to_string());
        let skip = !new_recipes.contains(&last_digit);
        recipes.extend(new_recipes.iter());
        elves_loc = elves_loc
            .iter()
            .map(|x| (x + recipes[*x] + 1) % recipes.len())
            .collect();

        if skip || recipes.len() < target_arr.len() + 1 {
            continue;
        }
        if recipes[recipes.len() - target_arr.len()..] == target_arr[..] {
            return recipes.len() - target_arr.len();
        }
        if recipes[recipes.len() - target_arr.len() - 1..recipes.len() - 1] == target_arr[..] {
            return recipes.len() - target_arr.len() - 1;
        }
    }
}

fn find_next_scores(raw_input: String, start: usize, length: usize) -> String {
    let mut recipes = parse_input(raw_input.clone());
    let mut elves_loc: Vec<usize> = vec![0, 1];

    while recipes.len() < start + length {
        let new_score: usize = elves_loc.iter().map(|x| recipes[*x]).sum();
        let mut new_recipes = parse_input(new_score.to_string());
        recipes.append(&mut new_recipes);

        elves_loc = elves_loc
            .iter()
            .map(|x| (x + recipes[*x] + 1) % recipes.len())
            .collect();
    }

    recipes[start..start + length]
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_initial_state() {
        let actual = parse_input("360781".to_string());
        assert_eq!(actual, [3, 6, 0, 7, 8, 1]);
    }

    #[test]
    fn should_find_next_n_scores() {
        let actual = find_next_scores("37".to_string(), 2018, 10);
        assert_eq!(actual, "5941429882");

        let actual = find_next_scores("37".to_string(), 9, 10);
        assert_eq!(actual, "5158916779");

        let actual = find_part_2("37".to_string(), 9);
        assert_eq!(actual, 13);

        let actual = find_next_scores("37".to_string(), 0, 20);
        assert_eq!(actual, "37101012451589167792");
    }

    #[test]
    fn should_solve_part_1() {
        let actual = find_next_scores("37".to_string(), 360781, 10);
        assert_eq!(actual, "6521571010");
    }

}
