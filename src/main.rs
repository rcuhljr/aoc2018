#![feature(test)]
extern crate test;
mod day1;
mod day2;
mod utility;

fn main() {
    println!("Day1a: {}", day1::solve_a());
    println!("Day1b: {}", day1::solve_b());
}
