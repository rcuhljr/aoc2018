use std::cmp;

pub fn solve_a() -> String {
    let res = locate_densest_area(5153);
    format!("{},{}", res.0, res.1)
}

pub fn solve_b() -> String {
    let res = locate_densest_var_area(5153, 300);
    format!("{},{},{}", res.0, res.1, res.2)
}

fn locate_densest_var_area(serial: i32, size: usize) -> (i32, i32, i32) {
    let grid = build_power_grid(serial, size as i32);
    let mut max = (0, 0, 0, 0);

    for x in 1..size + 1 {
        for y in 1..size + 1 {
            let mut sum = grid[x][y];
            let max_size = size + 1 - cmp::max(x, y);
            for i in 1..max_size {
                for j in 1..i + 1 {
                    sum += grid[x + i][y + j - 1];
                    sum += grid[x + j - 1][y + i];
                }
                sum += grid[x + i][y + i];
                if sum > max.2 {
                    max = (x, y, sum, i + 1);
                }
            }
        }
    }

    (max.0 as i32, max.1 as i32, max.3 as i32)
}

fn locate_densest_area(serial: i32) -> (i32, i32) {
    let grid = build_power_grid(serial, 300);
    let mut max = (0, 0, 0);

    for x in 1..298 {
        for y in 1..298 {
            let mut sum = 0;
            for i in 0..3 {
                for j in 0..3 {
                    sum += grid[x + i][y + j];
                }
            }
            if sum > max.2 {
                max = (x, y, sum);
            }
        }
    }

    (max.0 as i32, max.1 as i32)
}

fn build_power_grid(serial: i32, size: i32) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for x in 0..size + 1 {
        let mut col: Vec<i32> = Vec::new();
        for y in 0..size + 1 {
            col.push(calc_power(serial, x, y));
        }
        grid.push(col);
    }
    grid
}

fn calc_power(serial: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    (((rack_id * y + serial) * rack_id) % 1000) / 100 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample11a_for_time() {
        let actual = locate_densest_area(18);
        assert_eq!(actual, (33, 45));
    }

    #[test]
    fn should_solve_power_value() {
        let actual = calc_power(8, 3, 5);
        assert_eq!(actual, 4);
    }

    #[test]
    fn should_solve_power_grids() {
        let actual = build_power_grid(57, 300);
        assert_eq!(actual[122][79], -5);
        let actual = build_power_grid(39, 300);
        assert_eq!(actual[217][196], 0);
        let actual = build_power_grid(71, 300);
        assert_eq!(actual[101][153], 4);
    }

    #[test]
    fn should_solve_sample11_for_output() {
        let actual = locate_densest_var_area(18, 5);
        assert_eq!(actual, (1, 3, 3));
    }

}
