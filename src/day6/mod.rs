use super::utility;
use std::collections::HashMap;
use std::fmt;

pub fn solve_a() -> String {
    find_largest_area("input6.txt").to_string()
}

pub fn solve_b() -> String {
    find_safe_area("input6.txt", 10000).to_string()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct GridEntry {
    id: i32,
    dist: i32,
}

impl fmt::Display for GridEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.id >= 0 {
            write!(f, "{}", self.id)
        } else {
            write!(f, "{}", '.')
        }
    }
}

fn find_safe_area(filename: &str, cutoff: i32) -> i32 {
    let mut max_dim = 0;
    let mut index: i32 = 0;
    let mut goals: HashMap<(i32, i32), i32> = HashMap::new();
    utility::load_strings(filename.to_string())
        .iter()
        .for_each(|raw| {
            let v: Vec<i32> = raw
                .split(", ")
                .map(|val| val.trim().parse().unwrap())
                .collect();
            goals.insert((v[0], v[1]), index);
            if v[0] > max_dim {
                max_dim = v[0]
            }
            if v[1] > max_dim {
                max_dim = v[1]
            }
            index += 1;
        });

    let size: i32 = max_dim + 1;

    let grid = vec![0; (size * size) as usize];

    grid.iter()
        .enumerate()
        .map(|(index, x)| {
            let loc_x: i32 = (index as i32) % size;
            let loc_y: i32 = (index as i32) / size;
            if goals.keys().fold(0, |acc, coords| {
                acc + (coords.0 - loc_x).abs() + (coords.1 - loc_y).abs()
            }) < cutoff
            {
                1
            } else {
                0
            }
        }).fold(0, |acc, x| acc + x)
}

fn find_largest_area(filename: &str) -> i32 {
    let mut max_dim = 0;
    let mut index = 0;
    let mut goals: HashMap<(usize, usize), i32> = HashMap::new();
    let mut infinite = HashMap::new();
    utility::load_strings(filename.to_string())
        .iter()
        .for_each(|raw| {
            let v: Vec<usize> = raw
                .split(", ")
                .map(|val| val.trim().parse().unwrap())
                .collect();
            goals.insert((v[0], v[1]), index);
            if v[0] > max_dim {
                max_dim = v[0]
            }
            if v[1] > max_dim {
                max_dim = v[1]
            }
            index += 1;
        });
    let size = max_dim + 1;
    //owner id, distance, owner id -1 means tied
    let mut grid = vec![GridEntry { id: 0, dist: -1 }; size * size];
    for (coords, id) in goals.iter() {
        let (x, y) = *coords;
        let mut edges = vec![(x, y)];
        let mut distance = 0;
        visit_edge(&edges[0], distance, *id, size, &mut grid);
    }
    for (coords, id) in goals.iter() {
        let (x, y) = *coords;
        let mut edges = vec![(x, y)];
        let mut distance = 0;

        loop {
            let mut new_edges: Vec<(usize, usize)> = Vec::new();
            distance += 1;
            edges.iter().for_each(|edge| {
                let (loc_x, loc_y) = *edge;
                if loc_x >= x && loc_x < size - 1 {
                    let new_edge = (loc_x + 1, loc_y);
                    if !new_edges.contains(&new_edge) {
                        new_edges.push(new_edge);
                    }
                }
                if loc_x <= x && loc_x > 0 {
                    let new_edge = (loc_x - 1, loc_y);
                    if !new_edges.contains(&new_edge) {
                        new_edges.push(new_edge);
                    }
                }
                if loc_y >= y && loc_y < size - 1 {
                    let new_edge = (loc_x, loc_y + 1);
                    if !new_edges.contains(&new_edge) {
                        new_edges.push(new_edge);
                    }
                }
                if loc_y <= y && loc_y > 0 {
                    let new_edge = (loc_x, loc_y - 1);
                    if !new_edges.contains(&new_edge) {
                        new_edges.push(new_edge);
                    }
                }
            });
            if new_edges.len() == 0 {
                break;
            }
            edges = new_edges;
            edges
                .iter()
                .for_each(|edge| visit_edge(edge, distance, *id, size, &mut grid));
        }
    }
    let mut result = HashMap::new();
    for (index, entry) in grid.iter().enumerate() {
        if entry.id >= 0 {
            let counter = result.entry(entry.id).or_insert(0);
            *counter += 1;
            if index % size == 0
                || index % size == (size - 1)
                || index / size == 0
                || index / size == (size - 1)
            {
                infinite.insert(entry.id, true);
            }
        }
    }

    goals
        .values()
        .filter_map(|x| {
            if infinite.contains_key(x) {
                None
            } else {
                Some(*x)
            }
        }).map(|x| (x, *result.get(&x).unwrap()))
        .max_by_key(|x| x.1)
        .unwrap()
        .1
}
//owner id, distance, owner id -1 means tied
fn visit_edge(edge: &(usize, usize), dist: i32, id: i32, size: usize, grid: &mut Vec<GridEntry>) {
    let entry = &grid[edge.0 % size + edge.1 * size].clone();
    if entry.dist == -1 || entry.dist > dist {
        grid[edge.0 % size + edge.1 * size] = GridEntry { id, dist };
    } else if entry.dist == dist && entry.id != id {
        grid[edge.0 % size + edge.1 * size] = GridEntry { id: -1, dist };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_visit_edge() {
        let mut grid = vec![
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
        ];

        visit_edge(&(0, 0), 0, 1, 2, &mut grid);
        visit_edge(&(1, 1), 0, 2, 2, &mut grid);
        visit_edge(&(1, 0), 1, 1, 2, &mut grid);
        visit_edge(&(0, 1), 1, 1, 2, &mut grid);
        visit_edge(&(1, 0), 1, 2, 2, &mut grid);
        visit_edge(&(0, 1), 1, 2, 2, &mut grid);

        assert_eq!(
            grid,
            vec![
                GridEntry { id: 1, dist: 0 },
                GridEntry { id: -1, dist: 1 },
                GridEntry { id: -1, dist: 1 },
                GridEntry { id: 2, dist: 0 },
            ]
        )
    }

    #[test]
    fn should_claim_edge() {
        let mut grid = vec![
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
        ];

        visit_edge(&(1, 0), 1, 2, 2, &mut grid);
        visit_edge(&(1, 1), 2, 3, 2, &mut grid);
        visit_edge(&(0, 0), 3, 4, 2, &mut grid);
        visit_edge(&(0, 1), 4, 5, 2, &mut grid);
        visit_edge(&(1, 1), 1, 1, 2, &mut grid);

        assert_eq!(
            grid,
            vec![
                GridEntry { id: 4, dist: 3 },
                GridEntry { id: 2, dist: 1 },
                GridEntry { id: 5, dist: 4 },
                GridEntry { id: 1, dist: 1 },
            ]
        )
    }

    #[test]
    fn should_tie_edge() {
        let mut grid = vec![
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
            GridEntry { id: 0, dist: -1 },
        ];

        visit_edge(&(1, 0), 1, 2, 2, &mut grid);
        visit_edge(&(1, 0), 1, 3, 2, &mut grid);

        assert_eq!(
            grid,
            vec![
                GridEntry { id: 0, dist: -1 },
                GridEntry { id: -1, dist: 1 },
                GridEntry { id: 0, dist: -1 },
                GridEntry { id: 0, dist: -1 },
            ]
        )
    }

    #[test]
    fn should_solve_sample6a() {
        let actual: i32 = find_largest_area("./src/day6/test.txt");
        assert_eq!(actual, 17);
    }

    #[test]
    fn should_solve_sample6b() {
        let actual: i32 = find_safe_area("./src/day6/test.txt", 32);
        assert_eq!(actual, 16);
    }
}
