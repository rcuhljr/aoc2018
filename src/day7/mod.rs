use super::utility;
use std::cmp;
use std::collections::HashMap;

pub fn solve_a() -> String {
    find_path("input7.txt".to_string())
}

pub fn solve_b() -> String {
    time_path("input7.txt".to_string(), 60, 5)
}

fn time_path(filename: String, offset: i32, work_count: i32) -> String {
    let mut steps = parse_data(utility::load_strings(filename));
    let mut alphabet: Vec<char> = vec![];
    let mut parents: Vec<char> = vec![];

    steps.iter().for_each(|step| {
        alphabet.push(step.0);
        alphabet.push(step.1);
        parents.push(step.0);
    });

    alphabet.sort_unstable();
    alphabet.dedup();
    parents.sort_unstable();
    parents.dedup();

    let end_base: char = *alphabet
        .iter()
        .find(|letter| !parents.contains(letter))
        .unwrap();

    let mut workers = work_count;
    let mut done_at: HashMap<i32, Vec<char>> = HashMap::new();
    let mut timer = -1;
    let mut done: Vec<char> = vec![];
    let mut in_work: Vec<char> = vec![];
    loop {
        //pass time
        timer += 1;
        //finish any workers
        if done_at.contains_key(&timer) {
            for letter in done_at.get(&timer).unwrap() {
                done.push(*letter);
                in_work = in_work
                    .iter()
                    .filter(|x| *x != letter)
                    .map(|x| *x)
                    .collect();
                // println!("done: {:?} at {:?}", letter, timer);
                workers += 1;
            }
        }
        if done.len() == alphabet.len() {
            break;
        }
        if workers == 0 {
            continue;
        }
        //pick first N steps
        let blocks: Vec<char> = steps
            .iter()
            .filter(|x| !done.contains(&x.0))
            .map(|x| x.1)
            .collect();

        let mut possible_steps: Vec<char> = steps
            .iter()
            .filter(|letter| {
                !in_work.contains(&letter.0)
                    && !blocks.contains(&letter.0)
                    && !done.contains(&letter.0)
            }).map(|x| x.0)
            .collect();

        if possible_steps.len() == 0 {
            if in_work.len() == 0 && blocks.len() == 0 {
                possible_steps.push(end_base);
            } else {
                continue;
            }
        }

        possible_steps.sort_unstable();
        possible_steps.dedup();

        for index in 0..cmp::min(possible_steps.len(), workers as usize) {
            let mut b = [0; 1];
            possible_steps[index].encode_utf8(&mut b);
            let mut done_time = (b[0] - 64) as i32;
            done_time += offset + timer;

            let finished_at = done_at.entry(done_time).or_insert(vec![]);
            finished_at.push(possible_steps[index]);
            workers -= 1;
            in_work.push(possible_steps[index]);
        }
    }

    timer.to_string()
}

fn find_path(filename: String) -> String {
    let mut steps = parse_data(utility::load_strings(filename));
    let mut alphabet: Vec<char> = vec![];
    let mut parents: Vec<char> = vec![];

    steps.iter().for_each(|step| {
        alphabet.push(step.0);
        alphabet.push(step.1);
        parents.push(step.0);
    });

    alphabet.sort_unstable();
    alphabet.dedup();
    parents.sort_unstable();
    parents.dedup();

    let end_base: char = *alphabet
        .iter()
        .find(|letter| !parents.contains(letter))
        .unwrap();

    let mut unwrap_path: Vec<char> = vec![];

    unwrap_path.sort_unstable();

    for index in 0..alphabet.len() {
        let blocks: Vec<char> = steps
            .iter()
            .filter(|x| !unwrap_path.contains(&x.0))
            .map(|x| x.1)
            .collect();

        let mut possible_steps: Vec<(char, char)> = steps
            .iter()
            .filter(|letter| !blocks.contains(&letter.0) && !unwrap_path.contains(&letter.0))
            .map(|x| *x)
            .collect();

        let keep = possible_steps.iter().min_by_key(|x| x.0);
        if keep.is_some() {
            unwrap_path.push(keep.unwrap().0);
        } else {
            unwrap_path.push(end_base);
        }
    }

    unwrap_path.iter().collect()
}

fn parse_data(data: Vec<String>) -> Vec<(char, char)> {
    data.iter()
        .map(|raw| {
            let v: Vec<&str> = raw.split(" ").collect();
            (v[1].chars().next().unwrap(), v[7].chars().next().unwrap())
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample7a() {
        let actual = find_path("./src/day7/test.txt".to_string());
        assert_eq!(actual, "CABDFE".to_string());
    }

    #[test]
    fn should_utf() {
        let mut b = [0; 1];
        'Z'.encode_utf8(&mut b);
        assert_eq!(b[0], 90);
    }

    #[test]
    fn should_solve_sample7b() {
        let actual = time_path("./src/day7/test.txt".to_string(), 0, 2);
        assert_eq!(actual, "15".to_string());
    }
}
