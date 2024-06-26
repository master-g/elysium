use std::fmt::Display;

/// A tile in the Okey Mahjong game.
///
/// bits layout:
///
/// ```
/// |xxxbbbbb|bbbbbbbb|ccccrrrr|
/// ```
/// r: rank (1-13)
/// c: color (1-4)
/// b: bit (0-1)
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Tile {
	Red01 = 0b00000000_00000001_00010001,
	Red02 = 0b00000000_00000010_00010010,
	Red03 = 0b00000000_00000100_00010011,
	Red04 = 0b00000000_00001000_00010100,
	Red05 = 0b00000000_00010000_00010101,
	Red06 = 0b00000000_00100000_00010110,
	Red07 = 0b00000000_01000000_00010111,
	Red08 = 0b00000000_10000000_00011000,
	Red09 = 0b00000001_00000000_00011001,
	Red10 = 0b00000010_00000000_00011010,
	Red11 = 0b00000100_00000000_00011011,
	Red12 = 0b00001000_00000000_00011100,
	Red13 = 0b00010000_00000000_00011101,
	Yellow01 = 0b00000000_00000001_00100001,
	Yellow02 = 0b00000000_00000010_00100010,
	Yellow03 = 0b00000000_00000100_00100011,
	Yellow04 = 0b00000000_00001000_00100100,
	Yellow05 = 0b00000000_00010000_00100101,
	Yellow06 = 0b00000000_00100000_00100110,
	Yellow07 = 0b00000000_01000000_00100111,
	Yellow08 = 0b00000000_10000000_00101000,
	Yellow09 = 0b00000001_00000000_00101001,
	Yellow10 = 0b00000010_00000000_00101010,
	Yellow11 = 0b00000100_00000000_00101011,
	Yellow12 = 0b00001000_00000000_00101100,
	Yellow13 = 0b00010000_00000000_00101101,
	Blue01 = 0b00000000_00000001_01000001,
	Blue02 = 0b00000000_00000010_01000010,
	Blue03 = 0b00000000_00000100_01000011,
	Blue04 = 0b00000000_00001000_01000100,
	Blue05 = 0b00000000_00010000_01000101,
	Blue06 = 0b00000000_00100000_01000110,
	Blue07 = 0b00000000_01000000_01000111,
	Blue08 = 0b00000000_10000000_01001000,
	Blue09 = 0b00000001_00000000_01001001,
	Blue10 = 0b00000010_00000000_01001010,
	Blue11 = 0b00000100_00000000_01001011,
	Blue12 = 0b00001000_00000000_01001100,
	Blue13 = 0b00010000_00000000_01001101,
	Black01 = 0b00000000_00000001_10000001,
	Black02 = 0b00000000_00000010_10000010,
	Black03 = 0b00000000_00000100_10000011,
	Black04 = 0b00000000_00001000_10000100,
	Black05 = 0b00000000_00010000_10000101,
	Black06 = 0b00000000_00100000_10000110,
	Black07 = 0b00000000_01000000_10000111,
	Black08 = 0b00000000_10000000_10001000,
	Black09 = 0b00000001_00000000_10001001,
	Black10 = 0b00000010_00000000_10001010,
	Black11 = 0b00000100_00000000_10001011,
	Black12 = 0b00001000_00000000_10001100,
	Black13 = 0b00010000_00000000_10001101,
	Joker = 0b00011111_11111111_11111111,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
	Red,
	Yellow,
	Blue,
	Black,
	Joker,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rank {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Eleven,
	Twelve,
	Thirteen,
}

impl Tile {
	#[inline]
	pub fn color(&self) -> usize {
		(*self as usize) >> 4 & 0b00000000_00000000_00001111
	}

	#[inline]
	pub fn rank(&self) -> usize {
		(*self as usize) & 0b00000000_00000000_00001111
	}

	#[inline]
	pub fn value_without_bits(&self) -> usize {
		(*self as usize) & 0b00000000_00000000_11111111
	}

	#[inline]
	pub fn bit(&self) -> usize {
		(*self as usize) >> 8 & 0b00000000_00011111_11111111
	}

	pub fn new(color: Color, rank: Rank) -> Tile {
		let color_bits = match color {
			Color::Red => 0b00000000_00000000_00010000,
			Color::Yellow => 0b00000000_00000000_00100000,
			Color::Blue => 0b00000000_00000000_01000000,
			Color::Black => 0b00000000_00000000_10000000,
			Color::Joker => 0b00011111_11111111_11111111,
		};
		let other_bits = match rank {
			Rank::One => 0b00000000_00000001_00000001,
			Rank::Two => 0b00000000_00000010_00000010,
			Rank::Three => 0b00000000_00000100_00000011,
			Rank::Four => 0b00000000_00001000_00000100,
			Rank::Five => 0b00000000_00010000_00000101,
			Rank::Six => 0b00000000_00100000_00000110,
			Rank::Seven => 0b00000000_01000000_00000111,
			Rank::Eight => 0b00000000_10000000_00001000,
			Rank::Nine => 0b00000001_00000000_00001001,
			Rank::Ten => 0b00000010_00000000_00001010,
			Rank::Eleven => 0b00000100_00000000_00001011,
			Rank::Twelve => 0b00001000_00000000_00001100,
			Rank::Thirteen => 0b00010000_00000000_00001101,
		};

		Tile::try_from(color_bits | other_bits).unwrap()
	}
}

impl From<Tile> for usize {
	fn from(tile: Tile) -> usize {
		tile as usize
	}
}

impl TryFrom<usize> for Tile {
	type Error = &'static str;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0b00000000_00000001_00010001 => Ok(Tile::Red01),
			0b00000000_00000010_00010010 => Ok(Tile::Red02),
			0b00000000_00000100_00010011 => Ok(Tile::Red03),
			0b00000000_00001000_00010100 => Ok(Tile::Red04),
			0b00000000_00010000_00010101 => Ok(Tile::Red05),
			0b00000000_00100000_00010110 => Ok(Tile::Red06),
			0b00000000_01000000_00010111 => Ok(Tile::Red07),
			0b00000000_10000000_00011000 => Ok(Tile::Red08),
			0b00000001_00000000_00011001 => Ok(Tile::Red09),
			0b00000010_00000000_00011010 => Ok(Tile::Red10),
			0b00000100_00000000_00011011 => Ok(Tile::Red11),
			0b00001000_00000000_00011100 => Ok(Tile::Red12),
			0b00010000_00000000_00011101 => Ok(Tile::Red13),
			0b00000000_00000001_00100001 => Ok(Tile::Yellow01),
			0b00000000_00000010_00100010 => Ok(Tile::Yellow02),
			0b00000000_00000100_00100011 => Ok(Tile::Yellow03),
			0b00000000_00001000_00100100 => Ok(Tile::Yellow04),
			0b00000000_00010000_00100101 => Ok(Tile::Yellow05),
			0b00000000_00100000_00100110 => Ok(Tile::Yellow06),
			0b00000000_01000000_00100111 => Ok(Tile::Yellow07),
			0b00000000_10000000_00101000 => Ok(Tile::Yellow08),
			0b00000001_00000000_00101001 => Ok(Tile::Yellow09),
			0b00000010_00000000_00101010 => Ok(Tile::Yellow10),
			0b00000100_00000000_00101011 => Ok(Tile::Yellow11),
			0b00001000_00000000_00101100 => Ok(Tile::Yellow12),
			0b00010000_00000000_00101101 => Ok(Tile::Yellow13),
			0b00000000_00000001_01000001 => Ok(Tile::Blue01),
			0b00000000_00000010_01000010 => Ok(Tile::Blue02),
			0b00000000_00000100_01000011 => Ok(Tile::Blue03),
			0b00000000_00001000_01000100 => Ok(Tile::Blue04),
			0b00000000_00010000_01000101 => Ok(Tile::Blue05),
			0b00000000_00100000_01000110 => Ok(Tile::Blue06),
			0b00000000_01000000_01000111 => Ok(Tile::Blue07),
			0b00000000_10000000_01001000 => Ok(Tile::Blue08),
			0b00000001_00000000_01001001 => Ok(Tile::Blue09),
			0b00000010_00000000_01001010 => Ok(Tile::Blue10),
			0b00000100_00000000_01001011 => Ok(Tile::Blue11),
			0b00001000_00000000_01001100 => Ok(Tile::Blue12),
			0b00010000_00000000_01001101 => Ok(Tile::Blue13),
			0b00000000_00000001_10000001 => Ok(Tile::Black01),
			0b00000000_00000010_10000010 => Ok(Tile::Black02),
			0b00000000_00000100_10000011 => Ok(Tile::Black03),
			0b00000000_00001000_10000100 => Ok(Tile::Black04),
			0b00000000_00010000_10000101 => Ok(Tile::Black05),
			0b00000000_00100000_10000110 => Ok(Tile::Black06),
			0b00000000_01000000_10000111 => Ok(Tile::Black07),
			0b00000000_10000000_10001000 => Ok(Tile::Black08),
			0b00000001_00000000_10001001 => Ok(Tile::Black09),
			0b00000010_00000000_10001010 => Ok(Tile::Black10),
			0b00000100_00000000_10001011 => Ok(Tile::Black11),
			0b00001000_00000000_10001100 => Ok(Tile::Black12),
			0b00010000_00000000_10001101 => Ok(Tile::Black13),
			0b00011111_11111111_11111111 => Ok(Tile::Joker),
			_ => Err("Invalid tile"),
		}
	}
}

impl Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let color = match self.color() {
			0b1111 => {
				return f.write_str("j");
			}
			0b0001 => "r",
			0b0010 => "y",
			0b0100 => "b",
			0b1000 => "k",
			_ => "n/a",
		};

		let value = match self.rank() {
			1 => "1",
			2 => "2",
			3 => "3",
			4 => "4",
			5 => "5",
			6 => "6",
			7 => "7",
			8 => "8",
			9 => "9",
			10 => "10",
			11 => "11",
			12 => "12",
			13 => "13",
			_ => "",
		};
		f.write_str(&format!("{}{}", color, value))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
	Pair(Tile),
	Run(Vec<Tile>),
	Set(Vec<Tile>),
}
