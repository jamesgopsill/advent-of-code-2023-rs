use regex::Regex;

pub fn invoke(
	mut schematic: String,
	debug: bool,
) -> u32 {
	let mut sum: i32 = 0;
	let row_length = schematic.find("\n").unwrap() as i32;
	if debug {
		dbg!(row_length);
	}
	schematic = schematic.replace("\n", "");
	if debug {
		dbg!(&schematic);
	}
	// Get the symbol positions.
	let symbols_re = Regex::new(r"[^\d.]").unwrap();
	let symbol_indices: Vec<i32> = symbols_re
		.find_iter(&schematic)
		.map(|f| f.start() as i32)
		.collect();
	if debug {
		dbg!(&symbol_indices);
	}
	// Identify the numbers and see if they are
	// adjacent to a symbol
	let numbers_re = Regex::new(r"\d+").unwrap();
	let numbers = numbers_re.find_iter(&schematic);
	let mut neighbours: Vec<i32> = vec![];
	let mut remainder: i32;
	for number in numbers {
		if debug {
			println!("---");
		}
		neighbours.clear();
		let num = number.as_str().parse::<i32>().unwrap();
		if debug {
			dbg!(num);
		}
		let start_idx = number.start() as i32;
		remainder = start_idx % row_length;
		if remainder > 0 {
			neighbours.push(start_idx - 1);
			neighbours.push(start_idx - row_length - 1);
			neighbours.push(start_idx + row_length - 1);
		}
		let end_idx = number.end() as i32 - 1;
		if debug {
			dbg!(start_idx, end_idx);
		}
		remainder = end_idx % row_length;
		if remainder != 0 {
			neighbours.push(end_idx + 1);
			neighbours.push(end_idx - row_length + 1);
			neighbours.push(end_idx + row_length + 1);
		}
		for idx in start_idx..end_idx + 1 {
			neighbours.push(idx - row_length);
			neighbours.push(idx + row_length);
		}
		if debug {
			dbg!(&neighbours);
		}
		for n in &neighbours {
			if symbol_indices.contains(n) {
				if debug {
					println!("Neigbours a Symbol");
				}
				sum += num;
				break;
			}
		}
	}

	sum as u32
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/03x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 4361);
	}
}
