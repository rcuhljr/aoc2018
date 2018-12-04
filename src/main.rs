#![feature(test)]
extern crate test;
mod day1;
mod day2;
mod day3;
mod day4;
mod utility;

fn main() {
    println!("Day1a: {}", day1::solve_a());
    println!("Day1b: {}", day1::solve_b());
    println!("Day2a: {}", day2::solve_a());
    println!("Day2b: {}", day2::solve_b());
    println!("Day3a: {}", day3::solve_a());
    println!("Day3b: {}", day3::solve_b());
    println!("Day4a: {}", day4::solve_a());
    println!("Day4b: {}", day4::solve_b());
}
