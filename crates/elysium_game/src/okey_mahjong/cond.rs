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

#[cfg(test)]
mod tests {
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
			assert_eq!(super::okey_is_seven_pairs(&tiles), expected);
		}
	}
}
