use elysium_game::okey_mahjong::{okey_check_win, okey_tiles_from_str};

use crate::err::Error;

pub(super) async fn init() -> Result<(), Error> {
	use std::io::{self, BufRead};

	let stdin = io::stdin();
	let handle = stdin.lock();

	for line in handle.lines() {
		match line {
			Ok(line) => {
				let tiles = okey_tiles_from_str(&line);
				if tiles.is_empty() {
					continue;
				}
				let win = okey_check_win(&tiles);
				assert!(win, "{} should be winnable", line);
				println!("{} ✅", line);
			}
			Err(e) => eprintln!("Error reading line: {}", e),
		}
	}

	Ok(())
}
