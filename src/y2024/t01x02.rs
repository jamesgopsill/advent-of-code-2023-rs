pub fn invoke(input: String) -> u32 {
	let mut a: Vec<u32> = vec![];
	let mut b: Vec<u32> = vec![];

	for line in input.lines() {
		let mut elements = line.split_whitespace();
		a.push(elements.next().unwrap().parse::<u32>().unwrap());
		b.push(elements.next().unwrap().parse::<u32>().unwrap());
	}

	let mut similarity: u32 = 0;
	for x in a.iter() {
		let mut count: u32 = 0;
		for y in b.iter() {
			if x == y {
				count += 1;
			}
		}
		similarity += x * count;
	}

	similarity
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			"3   4
4   3
2   5
1   3
3   9
3   3
"
			.to_string(),
		);
		assert_eq!(result, 31);
	}
}
