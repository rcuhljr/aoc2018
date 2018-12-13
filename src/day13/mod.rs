use super::utility;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_a() -> String {
    let res = run_simulation("input13.txt".to_string(), false);
    format!("{},{}", res.0, res.1)
}

pub fn solve_b() -> String {
    let res = run_simulation("input13.txt".to_string(), true);
    format!("{},{}", res.0, res.1)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord)]
struct CartData {
    x: i32,
    y: i32,
    dir: char,
    turn: i32,
}

impl CartData {
    fn new(x: i32, y: i32, dir: char, turn: i32) -> CartData {
        CartData { x, y, dir, turn }
    }
}

impl PartialOrd for CartData {
    fn partial_cmp(&self, other: &CartData) -> Option<Ordering> {
        match self.y.cmp(&other.y) {
            Ordering::Equal => Some(self.x.cmp(&other.x)),
            other => Some(other),
        }
    }
}

fn run_simulation(filename: String, removal: bool) -> (i32, i32) {
    let (track, mut carts) = parse_input(filename);
    let mut cart_locs: HashMap<(i32, i32), bool> = HashMap::new();

    loop {
        cart_locs.clear();
        carts.iter().for_each(|cart| {
            cart_locs.insert((cart.x, cart.y), true);
        });
        carts.sort_unstable();

        let mut new_carts: Vec<CartData> = Vec::new();
        let mut dead_carts: Vec<(i32, i32)> = Vec::new();
        for cart in carts {
            let mut newx = cart.x;
            let mut newy = cart.y;
            let mut newdir = cart.dir;
            let mut newturn = cart.turn;
            if dead_carts
                .iter()
                .any(|data| data.0 == cart.x && data.1 == cart.y)
            {
                continue;
            }
            //find new loc

            match newdir {
                '<' => newx -= 1,
                '>' => newx += 1,
                'v' => newy += 1,
                '^' => newy -= 1,
                _ => (),
            }

            //check crash
            if cart_locs.contains_key(&(newx, newy)) {
                if removal {
                    dead_carts.push((newx, newy));
                    continue;
                } else {
                    return (newx, newy);
                }
            }
            //determine new facing
            let new_space = get_spot(newx, newy, &track);
            if new_space == '/' {
                if cart.dir == '<' {
                    newdir = 'v';
                } else if cart.dir == '>' {
                    newdir = '^';
                } else if cart.dir == 'v' {
                    newdir = '<';
                } else if cart.dir == '^' {
                    newdir = '>';
                }
            } else if new_space == '\\' {
                if cart.dir == '<' {
                    newdir = '^';
                } else if cart.dir == '>' {
                    newdir = 'v';
                } else if cart.dir == 'v' {
                    newdir = '>';
                } else if cart.dir == '^' {
                    newdir = '<';
                }
            } else if new_space == '+' {
                newturn += 1;
                if newturn % 3 == 0 {
                    if cart.dir == '<' {
                        newdir = '^';
                    } else if cart.dir == '>' {
                        newdir = 'v';
                    } else if cart.dir == 'v' {
                        newdir = '<';
                    } else if cart.dir == '^' {
                        newdir = '>';
                    }
                } else if newturn % 3 == 1 {
                    if cart.dir == '<' {
                        newdir = 'v';
                    } else if cart.dir == '>' {
                        newdir = '^';
                    } else if cart.dir == 'v' {
                        newdir = '>';
                    } else if cart.dir == '^' {
                        newdir = '<';
                    }
                }
            }
            cart_locs.remove_entry(&(cart.x, cart.y));
            cart_locs.insert((newx, newy), true);
            new_carts.push(CartData::new(newx, newy, newdir, newturn));
        }

        carts = new_carts
            .iter()
            .filter(|cart| {
                dead_carts
                    .iter()
                    .all(|data| data.0 != cart.x || data.1 != cart.y)
            }).cloned()
            .collect();

        if removal && carts.len() == 1 {
            return (carts[0].x, carts[0].y);
        }
    }
}

fn get_spot(x: i32, y: i32, track: &Vec<Vec<char>>) -> char {
    if (y >= track.len() as i32) || y < 0 || x < 0 || (x >= track[0].len() as i32) {
        ' '
    } else {
        track[y as usize][x as usize]
    }
}

fn is_intersection(x: i32, y: i32, track: &Vec<Vec<char>>) -> bool {
    get_spot(x, y, &track) == '+'
}

fn parse_input(filename: String) -> (Vec<Vec<char>>, Vec<CartData>) {
    let raw_data = utility::load_strings(filename);
    let mut track: Vec<Vec<char>> = Vec::new();
    let mut carts: Vec<CartData> = Vec::new();

    raw_data
        .iter()
        .for_each(|row| track.push(row.chars().collect()));

    let updown_symbols = vec!['/', '\\', '|', '+'];
    let leftright_symbols = vec!['/', '\\', '-', '+'];

    for y in 0..track.len() as i32 {
        for x in 0..track[y as usize].len() as i32 {
            let space = track[y as usize][x as usize];
            if space == '>' || space == '<' || space == 'v' || space == '^' {
                carts.push(CartData::new(x as i32, y as i32, space.clone(), 0));
                if is_intersection(x, y, &track) {
                    track[y as usize][x as usize] = '+';
                } else if leftright_symbols.contains(&get_spot(x - 1, y, &track))
                    && leftright_symbols.contains(&get_spot(x + 1, y, &track))
                {
                    track[y as usize][x as usize] = '-';
                } else if updown_symbols.contains(&get_spot(x, y - 1, &track))
                    && updown_symbols.contains(&get_spot(x, y + 1, &track))
                {
                    track[y as usize][x as usize] = '|';
                } else if (updown_symbols.contains(&get_spot(x, y - 1, &track))
                    && leftright_symbols.contains(&get_spot(x - 1, y, &track)))
                    || (updown_symbols.contains(&get_spot(x, y + 1, &track))
                        && leftright_symbols.contains(&get_spot(x + 1, y, &track)))
                {
                    track[y as usize][x as usize] = '/';
                } else if (updown_symbols.contains(&get_spot(x, y - 1, &track))
                    && leftright_symbols.contains(&get_spot(x + 1, y, &track)))
                    || (updown_symbols.contains(&get_spot(x, y + 1, &track))
                        && leftright_symbols.contains(&get_spot(x - 1, y, &track)))
                {
                    track[y as usize][x as usize] = '\\';
                }
            }
        }
    }

    (track, carts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_run_simulation_on_sample() {
        let actual = run_simulation("./src/day13/test.txt".to_string(), false);
        assert_eq!(actual, (7, 3));
    }

    #[test]
    fn should_run_simulation_on_sample_with_removal() {
        let actual = run_simulation("./src/day13/test2.txt".to_string(), true);
        assert_eq!(actual, (7, 4));
    }

    #[test]
    fn should_read_initial_state() {
        let actual = parse_input("./src/day13/test.txt".to_string());
        assert_eq!(
            actual.0,
            [
                ['/', '-', '-', '-', '\\', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['|', ' ', ' ', ' ', '|', ' ', ' ', '/', '-', '-', '-', '-', '\\'],
                ['|', ' ', '/', '-', '+', '-', '-', '+', '-', '\\', ' ', ' ', '|'],
                ['|', ' ', '|', ' ', '|', ' ', ' ', '|', ' ', '|', ' ', ' ', '|'],
                ['\\', '-', '+', '-', '/', ' ', ' ', '\\', '-', '+', '-', '-', '/'],
                [' ', ' ', '\\', '-', '-', '-', '-', '-', '-', '/', ' ', ' ', ' ']
            ]
        );

        assert_eq!(
            actual.1,
            [CartData::new(2, 0, '>', 0), CartData::new(9, 3, 'v', 0)]
        );
    }

    #[test]
    fn should_read_complex_initial_state() {
        let actual = parse_input("./src/day13/test3.txt".to_string());
        assert_eq!(
            actual.0,
            [
                ['/', '-', '-', '-', '\\', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['|', ' ', ' ', ' ', '|', ' ', ' ', '/', '-', '-', '-', '-', '\\'],
                ['|', ' ', '/', '-', '+', '-', '-', '+', '-', '\\', ' ', ' ', '|'],
                ['|', ' ', '|', ' ', '|', ' ', ' ', '|', ' ', '|', ' ', ' ', '|'],
                ['\\', '-', '+', '-', '/', ' ', ' ', '\\', '-', '+', '-', '-', '/'],
                [' ', ' ', '\\', '-', '-', '-', '-', '-', '-', '/', ' ', ' ', ' ']
            ]
        );
    }

}
