#![cfg_attr(not(feature = "harness"), no_std)]

extern crate alloc;

yaah::aoc_year!(2023);

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

yaah::aoc_lib!(with_benchmarks);
