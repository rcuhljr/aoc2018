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

    let mut graph = GraphMap::<char, (char, char), Directed>::from_edges(&steps);

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
                in_work = in_work.iter().cloned().filter(|x| x != letter).collect();
                workers += 1;
                graph.remove_node(*letter);
            }
        }
        if workers == 0 {
            continue;
        }
        let options = find_unblocked(&graph);
        if options.len() == 0 {
            if in_work.len() > 0 {
                continue;
            }
            break;
        }
        let ready: Vec<char> = options
            .iter()
            .cloned()
            .filter(|o| !in_work.contains(o))
            .collect();

        if ready.len() == 0 {
            continue;
        }

        for index in 0..cmp::min(ready.len(), workers as usize) {
            let done_time = char_to_offset_time(&ready[index], &offset, &timer);
            let finished_at = done_at.entry(done_time).or_insert(vec![]);
            finished_at.push(ready[index]);
            workers -= 1;
            in_work.push(ready[index]);
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
