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
pub fn okey_is_win(tiles: &[Tile]) -> bool {
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

	try_to_win(sorted)
}

#[inline]
fn check_sets_and_runs(tiles: &[Tile]) -> bool {
	debug_assert!(tiles.len() == 14); // comment out in release

	let full_mask = 16384; // 1 << 14
	let mut dp = vec![false; full_mask + 1];
	dp[0] = true;

	// iterate over all possible masks
	for mask in 0..=full_mask {
		if !dp[mask] {
			continue;
		}

		// try adding a new set or run
		for i in 0..14 {
			if mask & (1 << i) != 0 {
				// Tile i is already used
				continue;
			}

			// try to generate a set
			for j in (i + 1)..14 {
				if mask & (1 << j) != 0 {
					// Tile j is already used
					continue;
				}
				for k in (j + 1)..14 {
					if mask & (1 << k) != 0 {
						// Tile k is already used
						continue;
					}

					// try to generate a set of length 3
					let potential_set_3 = [tiles[i], tiles[j], tiles[k]];
					if okey_is_set(&potential_set_3) {
						let new_mask_3 = mask | (1 << i) | (1 << j) | (1 << k);
						dp[new_mask_3] = true;
						if new_mask_3 == full_mask {
							return true;
						}
					}

					// try to generate a set of length 4
					for l in (k + 1)..14 {
						if mask & (1 << l) != 0 {
							// Tile l is already used
							continue;
						}

						let potential_set_4 = [tiles[i], tiles[j], tiles[k], tiles[l]];
						if okey_is_run(&potential_set_4) {
							let new_mask_4 = mask | (1 << i) | (1 << j) | (1 << k) | (1 << l);
							dp[new_mask_4] = true;
							if new_mask_4 == full_mask {
								return true;
							}
						}
					}
				}
			}

			// try to generate a run
			for len in 3..=(14 - 1) {
				let end = i + len - 1;
				if end >= 14 || mask & (1 << end) != 0 {
					// Tile end is already used
					continue;
				}
				let mut new_mask = mask;
				let mut valid_run = true;
				for m in i..=end {
					if mask & (1 << m) != 0 {
						// Tile m is already used
						valid_run = false;
						break;
					}
					new_mask |= 1 << m;
				}
				if valid_run && okey_is_run(&tiles[i..=end]) {
					dp[new_mask] = true;
					if new_mask == full_mask {
						return true;
					}
				}
			}
		}
	}

	dp[full_mask]
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

fn try_to_win(mut free_tiles: Vec<Tile>) -> bool {
	let mut stack: Vec<SearchNode> = vec![SearchNode::new(free_tiles)];

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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::okey_mahjong::Tile;

	#[test]
	fn test_okey_is_seven_pairs() {
		assert!(okey_is_seven_pairs(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow06,
			Tile::Joker,
			Tile::Joker,
		]));
		assert!(okey_is_seven_pairs(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Yellow07,
		]));
		assert!(okey_is_seven_pairs(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Joker,
		]));
		assert!(okey_is_seven_pairs(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Joker,
			Tile::Joker,
		]));
		assert!(!okey_is_seven_pairs(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Yellow08,
		]));
		assert!(!okey_is_seven_pairs(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Yellow08,
		]));
	}

	#[test]
	fn test_okey_is_set() {
		assert!(!okey_is_set(&[Tile::Yellow01, Tile::Red01]));
		assert!(!okey_is_set(&[
			Tile::Yellow01,
			Tile::Yellow01,
			Tile::Red01,
			Tile::Black01,
			Tile::Red01
		]));
		assert!(!okey_is_set(&[Tile::Yellow01, Tile::Yellow01, Tile::Red01]));
		assert!(okey_is_set(&[Tile::Yellow01, Tile::Red01, Tile::Black01]));
		assert!(!okey_is_set(&[Tile::Yellow01, Tile::Yellow01, Tile::Joker]));
		assert!(okey_is_set(&[Tile::Yellow01, Tile::Joker, Tile::Joker]));
		assert!(okey_is_set(&[Tile::Yellow01, Tile::Red01, Tile::Blue01, Tile::Black01]));
		assert!(!okey_is_set(&[Tile::Yellow01, Tile::Red01, Tile::Blue01, Tile::Black02]));
		assert!(!okey_is_set(&[Tile::Yellow01, Tile::Red01, Tile::Black01, Tile::Black01]));
		assert!(okey_is_set(&[Tile::Yellow01, Tile::Red01, Tile::Joker, Tile::Black01]));
		assert!(okey_is_set(&[Tile::Yellow01, Tile::Red01, Tile::Joker, Tile::Joker]));
		assert!(!okey_is_set(&[
			Tile::Yellow01,
			Tile::Red01,
			Tile::Black01,
			Tile::Black01,
			Tile::Joker
		]));
	}

	#[test]
	fn test_okey_is_run() {
		assert!(okey_is_run(&[Tile::Yellow01, Tile::Yellow12, Tile::Yellow13,]));
		assert!(okey_is_run(&[
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Joker,
			Tile::Joker,
		]));
		assert!(!okey_is_run(&[Tile::Yellow01, Tile::Red01]));
		assert!(okey_is_run(&[
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Yellow08,
		]));
		assert!(okey_is_run(&[
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow03,
			Tile::Yellow04,
			Tile::Yellow05,
			Tile::Yellow06,
			Tile::Yellow07,
			Tile::Joker,
		]));
		assert!(okey_is_run(&[
			Tile::Yellow07,
			Tile::Yellow08,
			Tile::Yellow09,
			Tile::Yellow10,
			Tile::Yellow11,
			Tile::Yellow12,
			Tile::Yellow13,
			Tile::Yellow01,
		]));
		assert!(!okey_is_run(&[
			Tile::Yellow01,
			Tile::Yellow02,
			Tile::Yellow13,
			Tile::Yellow12,
			Tile::Yellow11,
			Tile::Yellow10,
			Tile::Joker,
			Tile::Joker,
		]));
		assert!(!okey_is_run(&[
			Tile::Yellow07,
			Tile::Yellow08,
			Tile::Yellow09,
			Tile::Yellow10,
			Tile::Yellow11,
			Tile::Yellow12,
			Tile::Yellow13,
			Tile::Black01,
		]));
		assert!(!okey_is_run(&[
			Tile::Black01,
			Tile::Black01,
			Tile::Black02,
			Tile::Black03,
			Tile::Black04,
		]));
		assert!(okey_is_run(&[
			Tile::Blue01,
			Tile::Blue02,
			Tile::Blue03,
			Tile::Blue04,
			Tile::Blue05,
			Tile::Blue06,
			Tile::Blue07,
			Tile::Blue08,
			Tile::Blue09,
			Tile::Blue10,
			Tile::Blue11,
			Tile::Blue12,
			Tile::Blue13,
		]));
		assert!(okey_is_run(&[Tile::Black12, Tile::Black13, Tile::Joker,]));
	}

	#[test]
	fn test_okey_is_win() {
		assert!(okey_is_win(&[
			Tile::Black01,
			Tile::Black02,
			Tile::Black03,
			Tile::Black04,
			Tile::Black05,
			Tile::Black06,
			Tile::Black07,
			Tile::Black08,
			Tile::Black09,
			Tile::Black10,
			Tile::Black11,
			Tile::Black12,
			Tile::Black13,
			Tile::Joker,
		]));
		assert!(okey_is_win(&[
			Tile::Black01,
			Tile::Black01,
			Tile::Black02,
			Tile::Black02,
			Tile::Black03,
			Tile::Black03,
			Tile::Black04,
			Tile::Black04,
			Tile::Black05,
			Tile::Black05,
			Tile::Black06,
			Tile::Black06,
			Tile::Black07,
			Tile::Black07,
		]));
		assert!(okey_is_win(&[
			Tile::Black01,
			Tile::Black01,
			Tile::Black02,
			Tile::Black02,
			Tile::Black03,
			Tile::Black03,
			Tile::Black04,
			Tile::Black04,
			Tile::Black05,
			Tile::Black05,
			Tile::Black06,
			Tile::Black06,
			Tile::Black07,
			Tile::Joker,
		]));
		assert!(okey_is_win(&[
			Tile::Black01,
			Tile::Black01,
			Tile::Black02,
			Tile::Black02,
			Tile::Black03,
			Tile::Black03,
			Tile::Black04,
			Tile::Black04,
			Tile::Black05,
			Tile::Black05,
			Tile::Black06,
			Tile::Black06,
			Tile::Joker,
			Tile::Joker,
		]));
	}
}
