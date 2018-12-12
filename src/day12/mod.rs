use super::utility;
use std::cmp;

pub fn solve_a() -> String {
    sum_living_plants("input12.txt".to_string(), 20).to_string()
}

pub fn solve_b() -> String {
    sum_living_plants("input12.txt".to_string(), 50000000000).to_string()
    // sum_living_plants("input12.txt".to_string(), 10).to_string()
}

fn sum_pots(pots: &[bool], offset: i32) -> i32 {
    let mut sum: i32 = 0;
    pots.iter().enumerate().for_each(|pair| {
        let (index, data) = pair;
        if *data {
            sum += index as i32 - offset;
        }
    });
    sum
}

fn pretty_print_row(pots: &[bool]) {
    println!(
        "{:?}",
        pots.iter()
            .map(|x| if *x { '#' } else { '.' })
            .collect::<String>()
    )
}

fn sum_living_plants(filename: String, gens: usize) -> i32 {
    let (state, rules) = get_initial_state_and_rules(filename);
    let mut offset = 4;
    let growth_factor = 1000;
    let mut pots = vec![false; offset];
    pots.append(&mut state.clone());
    pots.append(&mut vec![false; offset]);
    for i in 0..gens {
        if pots[pots.len() - 3] == true {
            // println!("growing");
            // println!("{:?}", pots);
            pots.append(&mut vec![false; growth_factor]);
            // println!("{:?}", pots);
        }
        if pots.iter().position(|x| *x == true).unwrap() > 102 {
            let mut new_pots = vec![false; pots.len() - 100];
            offset += 100;
            new_pots[..].clone_from_slice(&pots[100..]);
            pots = new_pots;
        }

        process_generation(&mut pots, &rules);
        if i % 1000000 == 0 {
            println!("million:{:?}", i / 1000000);
        }
        // pretty_print_row(&pots);
    }
    // println!("{:?}", pots);

    sum_pots(&pots, offset as i32)
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

fn process_generation(pots: &mut [bool], rules: &Vec<Vec<bool>>) {
    let mut updates: Vec<(usize, bool)> = vec![(0, false), (1, false)];

    pots.windows(5).enumerate().for_each(|pair| {
        let (index, data) = pair;
        if let Some(rule) = rules.iter().find(|rule| rule[0..5] == *data) {
            updates.push((index + 2, rule[5]));
        } else {
            updates.push((index + 2, false));
        }
    });

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
    fn should_apply_generation() {
        let mut actual = vec![false, true, true, true, false, false, false];

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
