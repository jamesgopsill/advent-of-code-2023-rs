use advent::{bench::bench, y2016::t06x02::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2016/06.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
