use super::utility;
use std::cmp;
use std::collections::VecDeque;

pub fn solve_a() -> String {
    sum_living_plants("input12.txt".to_string(), 20).to_string()
}

pub fn solve_b() -> String {
    sum_living_plants("input12.txt".to_string(), 50000000000).to_string()
    // sum_living_plants("input12.txt".to_string(), 10).to_string()
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

fn pretty_print_row(pots: &VecDeque<bool>, offset: i64) {
    println!(
        "{:?}:{:?}",
        offset,
        pots.iter()
            .map(|x| if *x { '#' } else { '.' })
            .collect::<String>()
    )
}

fn sum_living_plants(filename: String, gens: usize) -> i64 {
    let (state, rules) = get_initial_state_and_rules(filename);
    let mut offset: i64 = 10;
    let mut pots = VecDeque::new();
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    state.iter().for_each(|x| pots.push_back(*x));
    pots.push_back(false);
    pots.push_back(false);
    pots.push_back(false);
    for i in 1..gens + 1 {
        // pretty_print_row(&pots, offset);
        if pots[pots.len() - 3] == true {
            // println!("growing");
            // println!("{:?}", pots);
            pots.push_back(false);
            // println!("{:?}", pots);
        }
        if pots.len() > 200 {
            offset -= 1;
            pots.pop_front();
        }
        process_generation(&mut pots, &rules);

        if i % 100 == 0 {
            println!("{:?}:{:?}:{:?}", i, offset, sum_pots(&pots, offset));
            pretty_print_row(&pots, offset);
        }

        if i == 1000 {
            break;
        }
    }
    // pretty_print_row(&pots, offset);
    // println!("{:?}", pots);

    // pretty_print_row(&pots, offset);
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

    for index in 2..pots.len() - 2 {
        let data = vec![
            pots[index - 2],
            pots[index - 1],
            pots[index - 0],
            pots[index + 1],
            pots[index + 2],
        ];
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
    fn should_sum_pots() {
        let mut test = VecDeque::new();
        test.push_back(false);

        let actual = sum_pots(&test, -942);
        assert_eq!(actual, 5);
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
