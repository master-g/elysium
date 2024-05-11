use std::time::{Duration, Instant};

use clap::Args;
use elysium_game::okey_mahjong::{okey_check_win, okey_tiles_from_str, okey_tiles_to_string, Tile};
use rand::seq::SliceRandom;

use crate::err::Error;

#[derive(Args, Debug)]
pub(super) struct FooCommandArguments {}

pub(super) async fn init(_args: FooCommandArguments) -> Result<(), Error> {
	let str_deck = "
	r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13,
	r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13,
	y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13,
	y1, y2, y3, y4, y5, y6, y7, y8, y9, y10, y11, y12, y13,
	b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13,
	b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13,
	k1, k2, k3, k4, k5, k6, k7, k8, k9, k10, k11, k12, k13,
	k1, k2, k3, k4, k5, k6, k7, k8, k9, k10, k11, k12, k13,
	j, j
	";
	let mut deck = okey_tiles_from_str(str_deck);
	let mut rng = rand::thread_rng();

	let history_record: Duration = Duration::from_millis(118);

	// loop 1000 times
	loop {
		// pick random 14 tiles from deck
		deck.shuffle(&mut rng);
		let hand: Vec<Tile> = deck.iter().take(14).cloned().collect();
		let start = Instant::now();
		okey_check_win(&hand);
		let duration = start.elapsed();
		if duration > history_record {
			let desc = okey_tiles_to_string(&hand);
			println!("Most costest 14 tiles: {}, cost: {:?}", desc, duration);
			break;
		}
	}

	Ok(())
}
