use super::utility;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_a() -> String {
    determine_optimal_time("input4.txt".to_string()).to_string()
    //count_overlaps("input3.txt".to_string(), 1000).to_string()
}

pub fn solve_b() -> String {
    determine_optimal_time2("input4.txt".to_string()).to_string()
}

#[derive(Debug, Eq)]
struct Log {
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    data: String,
}

impl PartialEq for Log {
    fn eq(&self, other: &Log) -> bool {
        return self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.min == other.min
            && self.data == other.data;
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Log) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Log {
    fn cmp(&self, other: &Log) -> Ordering {
        if self.month.cmp(&other.month) != Ordering::Equal {
            self.month.cmp(&other.month)
        } else if self.day.cmp(&other.day) != Ordering::Equal {
            self.day.cmp(&other.day)
        } else if self.hour.cmp(&other.hour) != Ordering::Equal {
            self.hour.cmp(&other.hour)
        } else {
            self.min.cmp(&other.min)
        }
    }
}

fn parse_log(entry: &str) -> Log {
    let mut parts = entry.split(' ');
    let date: Vec<&str> = parts.next().unwrap().trim_matches('[').split('-').collect();
    let time: Vec<&str> = parts.next().unwrap().trim_matches(']').split(':').collect();
    parts.next();

    let data = parts.next().unwrap().to_string();
    let month: i32 = date[1].parse().unwrap();
    let day: i32 = date[2].parse().unwrap();
    let hour: i32 = time[0].parse().unwrap();
    let min: i32 = time[1].parse().unwrap();

    return Log {
        month,
        day,
        hour,
        min,
        data,
    };
}

fn determine_optimal_time(filename: String) -> i32 {
    let mut claims: Vec<Log> = utility::load_strings(filename)
        .iter()
        .map(|raw| parse_log(raw))
        .collect();

    claims.sort_unstable();

    let mut watches: HashMap<String, Vec<Vec<i32>>> = HashMap::new();
    let mut new_guard = false;
    let mut guard: String = "".to_string();
    let mut start_min = 0;

    for claim in claims {
        if claim.data.starts_with("#") {
            new_guard = true;
            guard = claim.data;
        } else if claim.data == "asleep" {
            start_min = claim.min;
        } else {
            let watch = watches.entry(guard.clone()).or_insert(Vec::new());
            if new_guard {
                new_guard = false;
                watch.push(vec![0; 60]);
            }
            let mut slots: Vec<i32> = watch.pop().unwrap();
            for index in start_min..claim.min {
                slots[index as usize] = 1;
            }
            watch.push(slots);
        }
    }

    let mut sleepiest = ("".to_string(), 0, 0);

    for (guard, watches) in watches.iter() {
        let mut totals = vec![0; 60];
        for day in watches.iter() {
            for index in 0..60 {
                totals[index] += day[index];
            }
        }
        let sum = totals.iter().fold(0, |acc, x| acc + x);
        if sum > sleepiest.1 {
            let max = totals.iter().max().unwrap();
            let loc_max: i32 = totals.iter().position(|&x| x == *max).unwrap() as i32;
            sleepiest = (guard.to_string(), sum, loc_max);
        }
    }
    let guard_val: i32 = (sleepiest.0).to_string().trim_matches('#').parse().unwrap();
    return guard_val * sleepiest.2;
}

fn determine_optimal_time2(filename: String) -> i32 {
    let mut claims: Vec<Log> = utility::load_strings(filename)
        .iter()
        .map(|raw| parse_log(raw))
        .collect();

    claims.sort_unstable();

    let mut watches: HashMap<String, Vec<Vec<i32>>> = HashMap::new();
    let mut new_guard = false;
    let mut guard: String = "".to_string();
    let mut start_min = 0;

    for claim in claims {
        if claim.data.starts_with("#") {
            new_guard = true;
            guard = claim.data;
        } else if claim.data == "asleep" {
            start_min = claim.min;
        } else {
            let watch = watches.entry(guard.clone()).or_insert(Vec::new());
            if new_guard {
                new_guard = false;
                watch.push(vec![0; 60]);
            }
            let mut slots: Vec<i32> = watch.pop().unwrap();
            for index in start_min..claim.min {
                slots[index as usize] = 1;
            }
            watch.push(slots);
        }
    }

    let mut sleepiest = ("".to_string(), 0, 0, 0);

    for (guard, watches) in watches.iter() {
        let mut totals = vec![0; 60];
        for day in watches.iter() {
            for index in 0..60 {
                totals[index] += day[index];
            }
        }
        let sum = totals.iter().fold(0, |acc, x| acc + x);
        let max = totals.iter().max().unwrap();
        let loc_max: i32 = totals.iter().position(|&x| x == *max).unwrap() as i32;
        if *max > sleepiest.3 {
            sleepiest = (guard.to_string(), sum, loc_max, *max);
        }
    }
    let guard_val: i32 = (sleepiest.0).to_string().trim_matches('#').parse().unwrap();
    return guard_val * sleepiest.2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn should_parse_entry() {
        let s = "[1518-11-04 00:02] Guard #99 begins shift";

        assert_eq!(
            parse_log(s),
            Log {
                month: 11,
                day: 4,
                hour: 0,
                min: 2,
                data: "#99".to_string()
            }
        );
    }

    #[test]
    fn should_sort_logs() {
        let first = Log {
            month: 11,
            day: 4,
            hour: 0,
            min: 2,
            data: "#1".to_string(),
        };
        let second = Log {
            month: 11,
            day: 4,
            hour: 0,
            min: 3,
            data: "#2".to_string(),
        };

        let mut actual = vec![second, first];

        actual.sort_unstable();

        assert_eq!(actual[0].data, "#1");
        assert_eq!(actual[1].data, "#2");
    }

    #[test]
    fn should_solve_sample() {
        let actual = determine_optimal_time("./src/day4/test_logs.txt".to_string());

        assert_eq!(actual, 240);
    }

    #[test]
    fn should_solve_sample2() {
        let actual = determine_optimal_time2("./src/day4/test_logs.txt".to_string());

        assert_eq!(actual, 4455);
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
