use super::utility;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use petgraph::Direction;
use std::cmp;
use std::collections::HashMap;

pub fn solve_a() -> String {
    find_path_graph("input7.txt".to_string())
}

pub fn solve_b() -> String {
    time_path("input7.txt".to_string(), 60, 5)
}

fn time_path(filename: String, offset: i32, work_count: i32) -> String {
    let steps = parse_data(utility::load_strings(filename));
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
        timer += 1;
        if done_at.contains_key(&timer) {
            for letter in done_at.get(&timer).unwrap() {
                done.push(*letter);
                in_work = in_work
                    .iter()
                    .filter(|x| *x != letter)
                    .map(|x| *x)
                    .collect();
                workers += 1;
            }
        }
        if done.len() == alphabet.len() {
            break;
        }
        if workers == 0 {
            continue;
        }

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
            let done_time = char_to_offset_time(&possible_steps[index], &offset, &timer);
            let finished_at = done_at.entry(done_time).or_insert(vec![]);
            finished_at.push(possible_steps[index]);
            workers -= 1;
            in_work.push(possible_steps[index]);
        }
    }

    timer.to_string()
}

fn char_to_offset_time(letter: &char, offset: &i32, timer: &i32) -> i32 {
    let mut b = [0; 1];
    letter.encode_utf8(&mut b);
    let result_time = (b[0] - 64) as i32;
    result_time + offset + timer
}

fn find_path_graph(filename: String) -> String {
    let steps = parse_data(utility::load_strings(filename));
    let mut graph = GraphMap::<char, (char, char), Directed>::from_edges(&steps);

    let mut result: Vec<char> = vec![];
    loop {
        let options = find_unblocked(&graph);
        if options.len() == 0 {
            break;
        }
        let remove = options.iter().min().unwrap();
        result.push(*remove);
        graph.remove_node(*remove);
    }

    result.iter().collect()
}

fn find_unblocked(graph: &GraphMap<char, (char, char), Directed>) -> Vec<char> {
    graph
        .nodes()
        .filter(|n| graph.neighbors_directed(*n, Direction::Incoming).count() == 0)
        .collect::<Vec<char>>()
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
        let actual = find_path_graph("./src/day7/test.txt".to_string());
        assert_eq!(actual, "CABDFE".to_string());
    }

    #[test]
    fn should_solve_sample7a_graph() {
        let actual = find_path_graph("./src/day7/test.txt".to_string());
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
