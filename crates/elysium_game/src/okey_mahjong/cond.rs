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
		if *count >= 2 {
			pair_count += 1;
		} else if *count == 1 && jokers > 0 {
			pair_count += 1;
			jokers -= 1;
		}
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

	false
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

				// try to generate a run
				for k in (j + 1)..14 {
					if mask & (1 << k) != 0 {
						// Tile k is already used
						continue;
					}

					let potential_set = [tiles[i], tiles[j], tiles[k]];
				}
			}
		}
	}

	dp[full_mask]
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::okey_mahjong::Tile;

	#[test]
	fn test_okey_is_seven_pairs() {
		let vectors = vec![
			(
				vec![
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
				],
				true,
			),
			(
				vec![
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
				],
				true,
			),
			(
				vec![
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
				],
				true,
			),
			(
				vec![
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
				],
				false,
			),
		];

		for (tiles, expected) in vectors {
			assert_eq!(okey_is_seven_pairs(&tiles), expected);
		}
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
	}
}
