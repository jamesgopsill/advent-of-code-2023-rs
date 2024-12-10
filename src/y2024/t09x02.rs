pub fn invoke(input: &String) -> u64 {
	let mut disk: Vec<Block> = vec![];
	let mut fid: u64 = 0;
	for (i, val) in input.trim().chars().enumerate() {
		let size = val.to_digit(10).unwrap() as u64;
		match i % 2 {
			0 => {
				let size = val.to_digit(10).unwrap() as u64;
				let file = File::new(fid, size.clone());
				let mut block = Block::new(size.clone());
				block.add_file(file);
				disk.push(block);
				fid += 1;
			}
			_ => {
				let block = Block::new(size.clone());
				disk.push(block);
			}
		}
	}
	// println!("{:?}", disk);
	// Something here toe re-position them
	for backwards in (0..disk.len()).rev() {
		let block = disk[backwards].clone();
		if block.files.len() == 1 {
			let file = block.files[0].clone();
			for forwards in 0..backwards {
				let mut block_before = disk[forwards].clone();
				let test = block_before.add_file(file.clone());
				if test {
					// println!("{} {:?} {}", backwards, file, forwards);
					disk[forwards] = block_before;
					disk[backwards].clear();
					break;
				}
			}
		}
	}
	// Now create the string
	let mut layout: Vec<u64> = vec![];
	for block in disk {
		layout.extend(block.layout());
	}
	// println!("{:?}", layout);

	// checksum
	let mut checksum: u64 = 0;
	for (i, val) in layout.iter().enumerate() {
		checksum += i as u64 * *val;
	}
	checksum
}

#[derive(Debug, Clone)]
struct File {
	fid: u64,
	size: u64,
}

impl File {
	fn new(
		fid: u64,
		size: u64,
	) -> Self {
		Self { fid, size }
	}
}

#[derive(Debug, Clone)]
struct Block {
	capacity_remaining: u64,
	files: Vec<File>,
}

impl Block {
	fn new(capacity: u64) -> Self {
		Self {
			capacity_remaining: capacity,
			files: vec![],
		}
	}

	fn add_file(
		&mut self,
		file: File,
	) -> bool {
		if self.capacity_remaining >= file.size {
			self.capacity_remaining -= file.size;
			self.files.push(file);
			return true;
		}
		return false;
	}

	fn clear(&mut self) {
		for file in self.files.iter() {
			self.capacity_remaining += file.size;
		}
		self.files.clear();
	}

	fn layout(&self) -> Vec<u64> {
		let mut layout: Vec<u64> = vec![];
		for file in self.files.iter() {
			for _ in 0..file.size {
				layout.push(file.fid);
			}
		}
		for _ in 0..self.capacity_remaining {
			layout.push(0);
		}
		layout
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "2333133121414131402".to_string();
		let result = invoke(&input);
		assert_eq!(result, 2858);
	}
}