use std::collections::HashSet;

pub fn invoke(input: &String) -> u32 {
	let mut map: Vec<Vec<char>> = vec![];
	for line in input.lines() {
		let mut row: Vec<char> = vec![];
		for c in line.chars() {
			row.push(c);
		}
		map.push(row);
	}

	let mut guard = Guard::new(map);
	guard.walk();
	guard.visited.len() as u32
}

#[derive(Debug)]
enum Facing {
	North,
	East,
	South,
	West,
}

struct Guard {
	map: Vec<Vec<char>>,
	position: [usize; 2],
	facing: Facing,
	visited: HashSet<[usize; 2]>,
}

impl Guard {
	fn new(map: Vec<Vec<char>>) -> Self {
		let position = find_starting_position(&map);
		let mut visited: HashSet<[usize; 2]> = HashSet::new();
		visited.insert(position.clone());
		Self {
			map,
			position,
			visited,
			facing: Facing::North,
		}
	}

	fn walk(&mut self) {
		let mut next: [usize; 2];
		let row_max = self.map.len() - 1;
		let col_max = self.map[0].len() - 1;
		loop {
			// println!("{:?} {:?}", self.position, self.facing);
			match self.facing {
				Facing::North => {
					// Arena check
					if self.position[0] == 0 {
						return;
					}
					next = self.position;
					next[0] -= 1;
					match self.map[next[0]][next[1]] {
						// Turn right
						'#' => self.facing = Facing::East,
						_ => self.position = next,
					}
				}
				Facing::East => {
					// Arena check
					if self.position[1] == col_max {
						return;
					}
					next = self.position;
					next[1] += 1;
					match self.map[next[0]][next[1]] {
						'#' => self.facing = Facing::South,
						_ => self.position = next,
					}
				}
				Facing::South => {
					// Arena check
					if self.position[0] == row_max {
						return;
					}
					next = self.position;
					next[0] += 1;
					match self.map[next[0]][next[1]] {
						'#' => self.facing = Facing::West,
						_ => self.position = next,
					}
				}
				Facing::West => {
					// Arena check
					if self.position[1] == 0 {
						return;
					}
					next = self.position;
					next[1] -= 1;
					match self.map[next[0]][next[1]] {
						// Turn right
						'#' => self.facing = Facing::North,
						_ => self.position = next,
					}
				}
			}
			self.visited.insert(self.position.clone());
		}
	}
}

fn find_starting_position(map: &Vec<Vec<char>>) -> [usize; 2] {
	let mut position: [usize; 2] = [0; 2];
	for (i, row) in map.iter().enumerate() {
		for (j, c) in row.iter().enumerate() {
			match c {
				'^' => {
					position[0] = i;
					position[1] = j;
					return position;
				}
				_ => {}
			}
		}
	}
	position
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
			.to_string(),
		);
		assert_eq!(result, 41);
	}
}
