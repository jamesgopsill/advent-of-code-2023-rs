use advent::{bench::bench, y2023::t02x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/02.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}