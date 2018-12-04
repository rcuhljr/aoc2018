use super::utility;

pub fn solve_a() -> String {
    //count_overlaps("input3.txt".to_string(), 1000).to_string()
    "".to_string()
}

pub fn solve_b() -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn should_do_something() {

        assert_eq!(1, 1);
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
