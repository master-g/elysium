use crate::okey_mahjong::{Color, Rank};

use super::Tile;

pub fn okey_tiles_from_str(s: &str) -> Vec<Tile> {
	let mut digits = String::new();

	let mut colors: Vec<Color> = Vec::new();
	let mut ranks: Vec<Rank> = Vec::new();

	let parse_digits = |d: &mut String, r: &mut Vec<Rank>| {
		if !d.is_empty() {
			if let Ok(rank_value) = d.parse::<usize>() {
				let rank = match rank_value {
					1 => Rank::One,
					2 => Rank::Two,
					3 => Rank::Three,
					4 => Rank::Four,
					5 => Rank::Five,
					6 => Rank::Six,
					7 => Rank::Seven,
					8 => Rank::Eight,
					9 => Rank::Nine,
					10 => Rank::Ten,
					11 => Rank::Eleven,
					12 => Rank::Twelve,
					13 => Rank::Thirteen,
					_ => {
						error!("Invalid rank: {}", d);
						return;
					}
				};
				trace!("pushing rank: {:?}", rank);
				r.push(rank);
				d.clear();
			} else {
				error!("Invalid rank: {}", d);
				d.clear();
			}
		}
	};

	for c in s.chars() {
		if c.is_ascii_digit() {
			// Number
			digits.push(c);
			continue;
		}

		let color = match c.to_ascii_lowercase() {
			'y' => {
				// Yellow
				Color::Yellow
			}
			'b' => {
				// Blue
				Color::Blue
			}
			'k' => {
				// Black
				Color::Black
			}
			'r' => {
				// Red
				Color::Red
			}
			'j' => {
				// Joker
				Color::Joker
			}
			_ => {
				continue;
			}
		};
		colors.push(color);

		parse_digits(&mut digits, &mut ranks);
		if color == Color::Joker {
			ranks.push(Rank::One);
		}
	}

	parse_digits(&mut digits, &mut ranks);

	if colors.len() != ranks.len() {
		error!("Colors and ranks have different lengths, {:?}, {:?}", colors, ranks);
		return Vec::new();
	}

	trace!("colors: {:?}", colors);
	trace!("ranks:  {:?}", ranks);
	colors.into_iter().zip(ranks).map(|(color, rank)| Tile::new(color, rank)).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_okey_tiles_from_str() {
		assert_eq!(okey_tiles_from_str("r13 j j"), vec![Tile::Red13, Tile::Joker, Tile::Joker]);
		assert_eq!(
			okey_tiles_from_str("r1 y2 b3 k4 j"),
			vec![
				Tile::new(Color::Red, Rank::One),
				Tile::new(Color::Yellow, Rank::Two),
				Tile::new(Color::Blue, Rank::Three),
				Tile::new(Color::Black, Rank::Four),
				Tile::Joker,
			]
		);

		assert_eq!(
			okey_tiles_from_str("r1 y2 b3 k4 j j"),
			vec![
				Tile::new(Color::Red, Rank::One),
				Tile::new(Color::Yellow, Rank::Two),
				Tile::new(Color::Blue, Rank::Three),
				Tile::new(Color::Black, Rank::Four),
				Tile::Joker,
				Tile::Joker,
			]
		);

		assert_eq!(
			okey_tiles_from_str("r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 j"),
			vec![
				Tile::Red01,
				Tile::Red02,
				Tile::Red03,
				Tile::Red04,
				Tile::Red05,
				Tile::Red06,
				Tile::Red07,
				Tile::Red08,
				Tile::Red09,
				Tile::Red10,
				Tile::Red11,
				Tile::Red12,
				Tile::Red13,
				Tile::Joker,
			]
		);
	}
}
