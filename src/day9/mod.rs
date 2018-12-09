use std::collections::LinkedList;

pub fn solve_a() -> String {
    play_game(419, 71052).1.to_string()
}

pub fn solve_b() -> String {
    play_game(419, 7105200).1.to_string()
}

fn move_cw<T>(list: &mut LinkedList<T>, offset: usize) {
    for _ in 0..offset {
        let temp = list.pop_front().unwrap();
        list.push_back(temp);
    }
}

fn move_ccw<T>(list: &mut LinkedList<T>, offset: usize) {
    for _ in 0..offset {
        let temp = list.pop_back().unwrap();
        list.push_front(temp);
    }
}

fn play_game(players: usize, last_marble: usize) -> (usize, usize) {
    let mut scores = vec![0; players];
    let mut circle: LinkedList<usize> = LinkedList::new();
    circle.push_front(0);

    for play in 1..last_marble + 1 {
        if play % 23 != 0 {
            move_cw(&mut circle, 2);
            circle.push_front(play);
        } else {
            move_ccw(&mut circle, 7);
            scores[play % players] += play + circle.pop_front().unwrap();
        }
    }

    scores
        .iter()
        .enumerate()
        .max_by_key(|score| *score.1)
        .map(|x| (x.0, *x.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample9a() {
        let game_score = play_game(9, 25);
        assert_eq!(game_score, (5, 32));
    }

    #[test]
    fn should_solve_another_sample9a() {
        let game_score = play_game(17, 1104);
        assert_eq!(game_score, (16, 2764));
    }

}
