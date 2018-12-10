use regex::Regex;
use utility;

pub fn solve_a() -> String {
    let mut data = parse_sky_data("input10.txt".to_string());
    solve_sky_data(&mut data).0.to_string()
}

pub fn solve_b() -> String {
    let mut data = parse_sky_data("input10.txt".to_string());
    solve_sky_data(&mut data).1.to_string()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RefFrame {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

impl RefFrame {
    pub fn new(minx: i32, maxx: i32, miny: i32, maxy: i32) -> RefFrame {
        RefFrame {
            minx,
            maxx,
            miny,
            maxy,
        }
    }

    pub fn width(&self) -> i32 {
        (self.maxx - self.minx) + 1
    }

    pub fn height(&self) -> i32 {
        (self.maxy - self.miny) + 1
    }

    pub fn offset_x(&self) -> i32 {
        self.minx * -1
    }

    pub fn offset_y(&self) -> i32 {
        self.miny * -1
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SkyData {
    position: Point,
    velocity: Point,
}

impl SkyData {
    pub fn new(x: i32, y: i32, xvel: i32, yvel: i32) -> SkyData {
        let position = Point { x, y };
        let velocity = Point { x: xvel, y: yvel };
        SkyData { position, velocity }
    }

    pub fn update_loc(&mut self) {
        self.position.update_x(self.velocity.x);
        self.position.update_y(self.velocity.y);
    }
}

impl Point {
    fn update_x(&mut self, adj: i32) {
        self.x += adj;
    }

    fn update_y(&mut self, adj: i32) {
        self.y += adj;
    }
}

fn solve_sky_data(sky_data: &mut Vec<SkyData>) -> (String, i32) {
    let mut frame = determine_next_frame(sky_data);
    let mut last_data;
    let mut last_frame;
    let mut time_counter = 0;

    loop {
        time_counter += 1;
        last_data = sky_data.clone();
        last_frame = frame.clone();
        frame = determine_next_frame(sky_data);
        if last_frame.width() + last_frame.height() <= frame.width() + frame.height() {
            break;
        }
    }

    let mut frame_data: Vec<Vec<char>> = Vec::new();
    for _ in 0..last_frame.height() {
        frame_data.push(vec!['.'; last_frame.width() as usize])
    }

    last_data.iter().for_each(|data| {
        frame_data[(data.position.y + last_frame.offset_y()) as usize]
            [(data.position.x + last_frame.offset_x()) as usize] = '#';
    });

    let result = frame_data
        .iter()
        .map(|line| line.iter().collect::<String>())
        .fold("\n".to_string(), |mut acc, x| {
            acc.push_str(&x);
            acc.push('\n');
            acc
        });

    (result, time_counter)
}

fn determine_next_frame(sky_data: &mut Vec<SkyData>) -> RefFrame {
    let (mut minx, mut miny, mut maxx, mut maxy) = (1000000, 1000000, -1000000, -1000000);
    sky_data.iter_mut().for_each(|data| {
        data.update_loc();
        if data.position.x < minx {
            minx = data.position.x;
        }
        if data.position.x > maxx {
            maxx = data.position.x;
        }
        if data.position.y < miny {
            miny = data.position.y;
        }
        if data.position.y > maxy {
            maxy = data.position.y;
        }
    });
    RefFrame::new(minx, maxx, miny, maxy)
}

fn parse_sky_data(filename: String) -> Vec<SkyData> {
    lazy_static! {
        static ref POINT_REGEX: Regex =
            Regex::new(r"<\s*(.*),\s*(.*)>.*<\s*(.*),\s*(.*)>").unwrap();
    }
    let raw_fields = utility::load_strings(filename.to_string());
    raw_fields
        .iter()
        .map(|raw| {
            let caps = POINT_REGEX.captures(raw).unwrap();
            SkyData::new(
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            )
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_sample10a_for_time() {
        let mut data = parse_sky_data("./src/day10/test.txt".to_string());
        let actual = solve_sky_data(&mut data);
        assert_eq!(actual.1, 3);
    }

    #[test]
    fn should_solve_sample10_for_output() {
        let mut data = parse_sky_data("./src/day10/test.txt".to_string());
        let actual = solve_sky_data(&mut data);
        assert_eq!(
            actual.0,
            "\n#...#..###\n#...#...#.\n#...#...#.\n#####...#.\n#...#...#.\n#...#...#.\n#...#...#.\n#...#..###\n"
                .to_string()
        );
    }

}
