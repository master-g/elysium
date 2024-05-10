use super::Tile;

#[inline]
pub fn okey_is_seven_pairs(tiles: &[Tile]) -> bool {
	let mut counts = [0usize; 256];

	for tile in tiles {
		counts[tile.value_without_bits()] += 1;
	}

	let mut pair_count = 0;
	let mut jokers = counts[Tile::Joker.value_without_bits()];

	for count in counts.iter().take(255) {
		match *count {
			0 => continue,
			1 => {
				if jokers > 0 {
					jokers -= 1;
					pair_count += 1;
				} else {
					return false;
				}
			}
			2 => pair_count += 1,
			_ => {
				error!("Invalid count: {}", count);
				return false;
			}
		}
	}

	if jokers == 2 {
		// If there are two jokers, they can be used as a pair
		pair_count += 1;
	} else if jokers == 1 {
		error!("Invalid joker count: {}", jokers);
		return false;
	}

	pair_count == 7
}

#[inline]
pub fn okey_is_set(tiles: &[Tile]) -> bool {
	// check length
	let origin_len = tiles.len();
	if !(3..=4).contains(&origin_len) {
		return false;
	}

	let filtered = tiles.iter().filter(|t| t != &&Tile::Joker).collect::<Vec<_>>();
	let num_of_jokers = origin_len - filtered.len();

	// Check if all tiles are the same rank, or if there are jokers
	if filtered.iter().any(|t| t.rank() != filtered[0].rank()) {
		return false;
	}

	// Check if there are origin_len - num_of_jokers colors
	let color_bits = filtered.iter().fold(0, |acc, &t| t.color() | acc);

	color_bits.count_ones() == (origin_len - num_of_jokers) as u32
}

#[inline]
pub fn okey_is_run(tiles: &[Tile]) -> bool {
	let origin_len = tiles.len();
	if origin_len < 3 {
		return false;
	}

	let filtered = tiles.iter().filter(|t| t != &&Tile::Joker).collect::<Vec<_>>();
	let num_of_jokers = origin_len - filtered.len();

	// should only have one color
	if filtered.iter().any(|t| t.color() != filtered[0].color()) {
		return false;
	}

	// should have origin_len - num_of_jokers bits
	let p_bits = filtered.iter().fold(0, |acc, &t| t.bit() | acc);
	if p_bits.count_ones() != (origin_len - num_of_jokers) as u32 {
		return false;
	}

	// these bits should be consecutive
	let is_continuous = |bits: usize, max_gaps: usize| -> bool {
		// remove trailing zeros
		let mut bits = bits >> bits.trailing_zeros();

		// count num of gaps
		let gaps = (bits.count_zeros() - bits.leading_zeros()) as usize;

		if gaps > max_gaps {
			return false;
		}

		while bits & (bits >> 1) != 0 {
			bits &= bits >> 1;
		}

		bits.count_ones() == 1
	};

	let check_bits = |bits: usize, jokers: usize| -> bool {
		// Check normal continuity
		if is_continuous(bits, jokers) {
			return true;
		}

		// Consider special case: 0b0000_0000_0000_0001 can be 0b0010_0000_0000_0000
		if p_bits & 0b0000_0000_0000_0001 != 0 {
			let modified_bits = (bits & !0b0000_0000_0000_0001) | 0b0010_0000_0000_0000;
			return is_continuous(modified_bits, jokers);
		}

		false
	};

	check_bits(p_bits, num_of_jokers)
}

#[inline]
pub fn okey_check_win(tiles: &[Tile]) -> bool {
	if tiles.len() != 14 {
		return false;
	}

	if okey_is_seven_pairs(tiles) {
		return true;
	}

	let sorted = {
		let mut sorted = tiles.to_vec();
		sorted.sort_by(|a, b| {
			let a = a.value_without_bits();
			let b = b.value_without_bits();
			a.cmp(&b)
		});
		sorted
	};

	// let sorted = tiles.to_vec();

	try_to_win(sorted)
}

struct SearchNode {
	sets: Vec<Vec<Tile>>,
	runs: Vec<Vec<Tile>>,
	free: Vec<Tile>,
}

impl SearchNode {
	fn new(free: Vec<Tile>) -> Self {
		Self {
			sets: Vec::new(),
			runs: Vec::new(),
			free,
		}
	}
}

fn try_to_win(tiles: Vec<Tile>) -> bool {
	let mut stack: Vec<SearchNode> = vec![SearchNode::new(tiles)];

	while let Some(current_node) = stack.pop() {
		if current_node.free.is_empty() {
			return true;
		}

		let n = current_node.free.len();
		for start in 0..n {
			for size in 3..=4 {
				if start + size > n {
					break;
				}
				let slice = &current_node.free[start..(start + size)];
				if okey_is_set(slice) {
					let mut new_free = current_node.free.clone();
					new_free.drain(start..(start + size));
					let mut new_node = SearchNode::new(new_free);
					new_node.sets.push(slice.to_vec());
					stack.push(new_node);
				}
			}

			for size in 3..=n - start {
				if start + size > n {
					break;
				}
				let slice = &current_node.free[start..(start + size)];
				if okey_is_run(slice) {
					let mut new_free = current_node.free.clone();
					new_free.drain(start..(start + size));
					let mut new_node = SearchNode::new(new_free);
					new_node.runs.push(slice.to_vec());
					stack.push(new_node);
				}
			}
		}
	}

	false
}
