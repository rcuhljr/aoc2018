use super::utility;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_a() -> String {
    determine_optimal_time_by_sum("input4.txt".to_string()).to_string()
}

pub fn solve_b() -> String {
    determine_optimal_time_by_freq("input4.txt".to_string()).to_string()
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Log {
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    data: String,
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

#[derive(Debug)]
struct WatchSummary {
    id: i32,
    max: i32,
    max_loc: i32,
    total: i32,
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

    Log {
        month,
        day,
        hour,
        min,
        data,
    }
}

fn determine_optimal_time_by_sum(filename: String) -> i32 {
    let mut claims: Vec<Log> = utility::load_strings(filename)
        .iter()
        .map(|raw| parse_log(raw))
        .collect();

    claims.sort_unstable();

    let watches = build_watches_map(claims);

    let summaries = build_watch_summary(watches);

    let max_freq = summaries
        .iter()
        .max_by_key(|summary| summary.total)
        .unwrap();

    max_freq.id * max_freq.max_loc
}

fn determine_optimal_time_by_freq(filename: String) -> i32 {
    let mut claims: Vec<Log> = utility::load_strings(filename)
        .iter()
        .map(|raw| parse_log(raw))
        .collect();

    claims.sort_unstable();

    let watches = build_watches_map(claims);

    let summaries = build_watch_summary(watches);

    let max_freq = summaries.iter().max_by_key(|summary| summary.max).unwrap();

    max_freq.id * max_freq.max_loc
}

fn build_watches_map(claims: Vec<Log>) -> HashMap<String, Vec<Vec<i32>>> {
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
    watches
}

fn build_watch_summary(watches: HashMap<String, Vec<Vec<i32>>>) -> Vec<WatchSummary> {
    let mut summaries: Vec<WatchSummary> = Vec::new();

    for (guard, watches) in watches.iter() {
        let mut totals = vec![0; 60];
        for day in watches.iter() {
            for index in 0..60 {
                totals[index] += day[index];
            }
        }
        let total = totals.iter().fold(0, |acc, x| acc + x);
        let max: i32 = *totals.iter().max().unwrap();
        let max_loc: i32 = totals.iter().position(|&x| x == max).unwrap() as i32;
        let id: i32 = guard.trim_matches('#').parse().unwrap();
        summaries.push(WatchSummary {
            id,
            max,
            max_loc,
            total,
        });
    }
    summaries
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn should_solve_sample4a() {
        let actual = determine_optimal_time_by_sum("./src/day4/test.txt".to_string());

        assert_eq!(actual, 240);
    }

    #[test]
    fn should_solve_sample4b() {
        let actual = determine_optimal_time_by_freq("./src/day4/test.txt".to_string());

        assert_eq!(actual, 4455);
    }
}
