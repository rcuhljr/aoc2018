use super::utility;

pub fn solve_a() -> String {
    return count_overlaps("input3.txt".to_string(), 1000).to_string();
}

pub fn solve_b() -> String {
    return find_no_overlaps("input3.txt".to_string());
}

#[derive(Debug)]
struct Claim {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    lrx: i32,
    lry: i32,
}

impl PartialEq for Claim {
    fn eq(&self, other: &Claim) -> bool {
        return self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height;
    }
}

fn parse_claim(raw_claim: String) -> Claim {
    let mut parts = raw_claim.split(' ').skip(2);
    let xy: Vec<&str> = parts.next().unwrap().trim_matches(':').split(',').collect();
    let x: i32 = xy[0].parse().unwrap();
    let y: i32 = xy[1].parse().unwrap();
    let dims: Vec<&str> = parts.next().unwrap().split('x').collect();
    let width: i32 = dims[0].parse().unwrap();
    let height: i32 = dims[1].parse().unwrap();
    let lrx = x + width - 1;
    let lry = y + height - 1;

    return Claim {
        x,
        y,
        width,
        height,
        lrx,
        lry,
    };
}

fn count_overlaps(filename: String, dims: i32) -> i32 {
    let claims: Vec<Claim> = utility::load_strings(filename)
        .iter()
        .map(|raw| parse_claim(raw.to_string()))
        .collect();
    let size = dims * dims;
    let mut field = vec![0; size as usize];

    claims
        .iter()
        .for_each(|claim| add_claim(claim, &mut field, dims));

    return field.iter().fold(0, |a, b| if b > &1 { a + 1 } else { a });
}

fn add_claim(claim: &Claim, field: &mut [i32], dims: i32) {
    for col in 0..claim.width {
        for row in 0..claim.height {
            let abs_pos = (row + claim.y) * dims + col + claim.x;
            field[abs_pos as usize] += 1;
        }
    }
}

fn find_no_overlaps(filename: String) -> String {
    let claims: Vec<Claim> = utility::load_strings(filename)
        .iter()
        .map(|raw| parse_claim(raw.to_string()))
        .collect();
    let mut valids = vec![true; claims.len()];
    for index in 0..valids.len() {
        if !valids[index] {
            continue;
        }
        for inner in 0..valids.len() {
            if inner == index {
                continue;
            }
            if claims_overlap(&claims[index], &claims[inner]) {
                valids[index] = false;
                valids[inner] = false;
            }
        }
    }
    return (valids.iter().position(|&x| x).unwrap() + 1).to_string();
}

fn claims_overlap(left: &Claim, right: &Claim) -> bool {
    if left.lrx < right.x || right.lrx < left.x {
        return false;
    }

    if left.lry < right.y || right.lry < left.y {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn should_parse_claim_string() {
        let v = "#1 @ 1,3: 4x4".to_string();
        let expected = Claim {
            x: 1,
            y: 3,
            width: 4,
            height: 4,
            lrx: 4,
            lry: 6,
        };

        assert_eq!(parse_claim(v), expected);
    }

    #[test]
    fn should_increment_by_claim() {
        let sample = Claim {
            x: 1,
            y: 1,
            width: 1,
            height: 1,
            lrx: 0,
            lry: 0,
        };
        let mut actual = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = [0, 0, 0, 0, 1, 0, 0, 0, 0];

        add_claim(&sample, &mut actual, 3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_solve_sample() {
        assert_eq!(
            count_overlaps("./src/day3/test_claims.txt".to_string(), 8),
            4
        );
    }

    #[test]
    fn should_solve_clean_sample() {
        assert_eq!(
            find_no_overlaps("./src/day3/test_claims.txt".to_string()),
            "3"
        );
    }

    #[bench]
    fn bench_a(b: &mut Bencher) {
        b.iter(|| solve_a());
    }

    #[bench]
    fn bench_b(b: &mut Bencher) {
        b.iter(|| solve_b());
    }
}
