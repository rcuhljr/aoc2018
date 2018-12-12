use super::utility;
use std::collections::VecDeque;

pub fn solve_a() -> String {
    sum_living_plants("input12.txt".to_string(), 20).to_string()
}

pub fn solve_b() -> String {
    sum_living_plants("input12.txt".to_string(), 50000000000).to_string()
}

fn sum_pots(pots: &VecDeque<bool>, offset: i64) -> i64 {
    let mut sum: i64 = 0;
    pots.iter().enumerate().for_each(|pair| {
        let (index, data) = pair;
        if *data {
            sum += index as i64 - offset;
        }
    });
    sum
}

fn sum_living_plants(filename: String, gens: i64) -> i64 {
    let (state, rules) = get_initial_state_and_rules(filename);
    let mut offset: i64 = 10;
    let convergance_step = 25;
    let mut pots = VecDeque::new();
    let mut old_scores = (0, 0, 0);
    for _ in 0..offset {
        pots.push_back(false);
    }
    state.iter().for_each(|x| pots.push_back(*x));
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);

    for i in 1..gens + 1 {
        if pots[pots.len() - 3] == true {
            pots.push_back(false);
        }
        if pots.len() > 200 {
            offset -= 1;
            pots.pop_front();
        }
        process_generation(&mut pots, &rules);

        if i % convergance_step == 0 {
            old_scores = (old_scores.1, old_scores.2, sum_pots(&pots, offset));
            if old_scores.2 - old_scores.1 == old_scores.1 - old_scores.0 {
                let delta_per_step = old_scores.2 - old_scores.1;
                return old_scores.2 + (gens - i as i64) / convergance_step * delta_per_step as i64;
            }
        }
    }

    sum_pots(&pots, offset as i64)
}

fn get_initial_state_and_rules(filename: String) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut raw_data = utility::load_strings(filename);
    let state_string = raw_data.remove(0);

    let raw_states: Vec<&str> = state_string.split(' ').collect();
    let states = raw_states[2].chars().map(|x| x == '#').collect();

    let rules = raw_data
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.clone().into_bytes())
        .map(|b| {
            vec![
                b[0] as char == '#',
                b[1] as char == '#',
                b[2] as char == '#',
                b[3] as char == '#',
                b[4] as char == '#',
                b[9] as char == '#',
            ]
        }).collect();

    (states, rules)
}

fn process_generation(pots: &mut VecDeque<bool>, rules: &Vec<Vec<bool>>) {
    let mut updates: Vec<(usize, bool)> = vec![(0, false), (1, false)];
    let reader: Vec<bool> = pots.iter().cloned().collect();

    for index in 2..pots.len() - 2 {
        let data = &reader[index - 2..index + 3];
        if let Some(rule) = rules.iter().find(|rule| rule[0..5] == data[..]) {
            updates.push((index, rule[5]));
        } else {
            updates.push((index, false));
        }
    }

    updates.iter().for_each(|pair| pots[pair.0] = pair.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample() {
        let actual = sum_living_plants("./src/day12/test.txt".to_string(), 20);
        assert_eq!(actual, 325);
    }

    #[test]
    fn should_solve_realday12() {
        let actual = sum_living_plants("input12.txt".to_string(), 50000000000);
        assert_eq!(actual, 2550000000883);
    }

    #[test]
    fn should_apply_generation() {
        let mut actual = VecDeque::new();
        actual.push_back(false);
        actual.push_back(true);
        actual.push_back(true);
        actual.push_back(true);
        actual.push_back(false);
        actual.push_back(false);
        actual.push_back(false);

        process_generation(
            &mut actual,
            &vec![
                vec![false, true, true, true, false, false],
                vec![true, true, false, false, false, true],
            ],
        );
        assert_eq!(actual, [false, false, false, false, true, false, false]);
    }

    #[test]
    fn should_read_initial_state() {
        let actual = get_initial_state_and_rules("./src/day12/test.txt".to_string());
        assert_eq!(
            actual.0,
            vec![
                true, false, false, true, false, true, false, false, true, true, false, false,
                false, false, false, false, true, true, true, false, false, false, true, true,
                true
            ]
        );
        assert_eq!(
            actual.1,
            [
                [false, false, false, true, true, true],
                [false, false, true, false, false, true],
                [false, true, false, false, false, true],
                [false, true, false, true, false, true],
                [false, true, false, true, true, true],
                [false, true, true, false, false, true],
                [false, true, true, true, true, true],
                [true, false, true, false, true, true],
                [true, false, true, true, true, true],
                [true, true, false, true, false, true],
                [true, true, false, true, true, true],
                [true, true, true, false, false, true],
                [true, true, true, false, true, true],
                [true, true, true, true, false, true]
            ]
        );
    }

}
