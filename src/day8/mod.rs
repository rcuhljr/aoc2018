use super::utility;

pub fn solve_a() -> String {
    calc_checksum("input8.txt".to_string()).to_string()
}

pub fn solve_b() -> String {
    calc_value("input8.txt".to_string()).to_string()
}

#[derive(Debug, Clone)]
struct TNode {
    id: usize,
    metadata: Vec<i32>,
    children: Vec<TNode>,
}

impl TNode {
    pub fn new(id: usize, metadata: Vec<i32>, children: Vec<TNode>) -> TNode {
        TNode {
            id,
            metadata,
            children,
        }
    }

    pub fn check_sum(&self) -> i32 {
        self.metadata.iter().fold(0, |acc, x| acc + x) + self
            .children
            .iter()
            .map(|x| x.check_sum())
            .fold(0, |acc, x| acc + x)
    }

    pub fn value(&self) -> i32 {
        if self.children.len() == 0 {
            self.metadata.iter().fold(0, |acc, x| acc + x)
        } else {
            self.metadata.iter().fold(0, |acc, x| {
                if x - 1 >= self.children.len() as i32 {
                    acc
                } else {
                    acc + self.children[(x - 1) as usize].value()
                }
            })
        }
    }
}

fn calc_value(filename: String) -> i32 {
    parse_tree(filename).value()
}

fn calc_checksum(filename: String) -> i32 {
    parse_tree(filename).check_sum()
}

fn parse_tree(filename: String) -> TNode {
    let mut raw_entries = parse_data(&mut utility::load_strings(filename));
    parse_entry_rec(0, &mut raw_entries).0
}

fn parse_entry_rec(id: usize, raw_entries: &mut Vec<i32>) -> (TNode, Vec<i32>) {
    let child_count = raw_entries[0];
    let meta_count = raw_entries[1] as usize;

    let mut children = vec![];
    let mut remainder = raw_entries.split_off(2);
    if child_count == 0 {
        let mut entries: Vec<i32> = vec![0; meta_count];
        entries.copy_from_slice(&remainder[0..meta_count]);
        let remainder = remainder.split_off(meta_count);

        (TNode::new(id, entries, children), remainder)
    } else {
        for i in 0..child_count {
            let result = parse_entry_rec(id + i as usize + 1, &mut remainder);
            children.push(result.0);
            remainder = result.1;
        }
        let mut entries: Vec<i32> = vec![0; meta_count];
        entries.copy_from_slice(&remainder[0..meta_count]);
        remainder = remainder.split_off(meta_count);

        (TNode::new(id, entries, children), remainder)
    }
}

fn parse_data(data: &mut Vec<String>) -> Vec<i32> {
    data.pop()
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample8a() {
        let actual = calc_checksum("./src/day8/test.txt".to_string());
        assert_eq!(actual, 138);
    }

    #[test]
    fn should_solve_sample8b() {
        let actual = calc_value("./src/day8/test.txt".to_string());
        assert_eq!(actual, 66);
    }
}
