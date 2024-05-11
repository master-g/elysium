use core::num;

use crate::okey_mahjong::okey_tiles_to_string;

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
				error!("impossible count of {}, {:?}", count, tiles);
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

pub fn okey_is_set(tiles: &[Tile]) -> bool {
	// check length
	let origin_len = tiles.len();
	if !(3..=4).contains(&origin_len) {
		return false;
	}

	is_set_fast(tiles)
}

#[inline]
fn is_set_fast(tiles: &[Tile]) -> bool {
	let origin_len = tiles.len();
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

pub fn okey_is_run(tiles: &[Tile]) -> bool {
	// check length
	let origin_len = tiles.len();
	if !(3..=13).contains(&origin_len) {
		return false;
	}

	is_run_fast(tiles)
}

#[inline]
fn is_run_fast(tiles: &[Tile]) -> bool {
	let origin_len = tiles.len();
	let mut p_bits = 0;
	let mut color = None;
	let mut num_of_jokers = 0;

	for &tile in tiles {
		if tile == Tile::Joker {
			num_of_jokers += 1;
		} else {
			if let Some(c) = color {
				if c != tile.color() {
					return false;
				}
			} else {
				color = Some(tile.color());
			}
			p_bits |= tile.bit();
		}
	}

	let num_needed = origin_len - num_of_jokers;
	let num_bits = p_bits.count_ones() as usize;
	if num_bits != num_needed {
		return false;
	}

	// these bits should be consecutive
	let is_continuous = |bits: usize, max_gaps: usize| -> bool {
		// remove trailing zeros
		let bits = bits >> bits.trailing_zeros();

		// count num of gaps
		let gaps = (bits.count_zeros() - bits.leading_zeros()) as usize;

		gaps <= max_gaps
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
	debug_assert!(tiles.len() == 14, "tiles: {} length is not 14", okey_tiles_to_string(tiles));

	if okey_is_seven_pairs(tiles) {
		return true;
	}

	try_to_win(tiles)
}

fn try_to_win(tiles: &[Tile]) -> bool {
	let big_num = 1 << 14;
	let mut dp = vec![false; big_num];
	let mut subset = [Tile::Joker; 14];

	dp[0] = true;
	for mask in 0..big_num {
		if !dp[mask] {
			continue;
		}
		// iterate all possible subsets
		for submask in 1..big_num {
			if (mask & submask) != 0 {
				continue;
			}
			if submask.count_ones() < 3 {
				continue;
			}
			let subset_len = (0..14).filter(|&i| (submask >> i) & 1 == 1).fold(0, |acc, i| {
				subset[acc] = tiles[i];
				acc + 1
			});
			if is_set_fast(&subset[..subset_len]) || is_run_fast(&subset[..subset_len]) {
				dp[mask | submask] = true;
				if mask | submask == big_num - 1 {
					return true;
				}
			}
		}
	}

	dp[big_num - 1]
}
