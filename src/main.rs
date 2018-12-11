#[macro_use]
extern crate lazy_static;
extern crate petgraph;
extern crate regex;
use std::time::Instant;
mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utility;

fn record_times(format: &str, f: &Fn() -> String) {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{}: {} \n\t\tsolved in: {:?}", format, result, duration);
}

fn main() {
    record_times("Day1a", &day1::solve_a);
    record_times("Day1b", &day1::solve_b);
    record_times("Day2a", &day2::solve_a);
    record_times("Day2b", &day2::solve_b);
    record_times("Day3a", &day3::solve_a);
    record_times("Day3b", &day3::solve_b);
    record_times("Day4a", &day4::solve_a);
    record_times("Day4b", &day4::solve_b);
    record_times("Day5a", &day5::solve_a);
    record_times("Day5b", &day5::solve_b);
    record_times("Day6a", &day6::solve_a);
    record_times("Day6b", &day6::solve_b);
    record_times("Day7a", &day7::solve_a);
    record_times("Day7b", &day7::solve_b);
    record_times("Day8a", &day8::solve_a);
    record_times("Day8b", &day8::solve_b);
    record_times("Day9a", &day9::solve_a);
    record_times("Day9b", &day9::solve_b);
    record_times("Day10a", &day10::solve_a);
    record_times("Day10b", &day10::solve_b);
    record_times("Day11a", &day11::solve_a);
    record_times("Day11b", &day11::solve_b);
}
