#[cfg(test)]
mod tests {
	use std::time::{Duration, Instant};

	use elysium_game::okey_mahjong::*;
	use rand::seq::SliceRandom;

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

	#[test_log::test]
	fn test_okey_is_run() {
		assert!(okey_is_run(&[Tile::Red07, Tile::Joker, Tile::Red09]));
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
	fn test_okey_check_win() {
		let mut rnd = rand::thread_rng();
		let mut vec1 = okey_tiles_from_str("b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 j");
		vec1.shuffle(&mut rnd);
		assert!(okey_check_win(&vec1));

		let mut vec2 = okey_tiles_from_str("b1 b1 r2 r2 y5 y5 b6 b6 b7 b7 b8 b8 k9 k9");
		vec2.shuffle(&mut rnd);
		assert!(okey_check_win(&vec2));

		let mut vec3 = okey_tiles_from_str("b1 b1 b2 b2 k3 k3 k4 k4 r5 r5 y6 y6 k7 j");
		vec3.shuffle(&mut rnd);
		assert!(okey_check_win(&vec3));

		let mut vec4 = okey_tiles_from_str("b1 b1 b2 b2 k3 k3 k4 k4 r5 r5 y6 y6 j j");
		vec4.shuffle(&mut rnd);
		assert!(okey_check_win(&vec4));

		let mut vec5 = okey_tiles_from_str("r1 y1 b1 k1 k2 k3 r1 r2 r3 r4 r5 r6 r7 r8");
		vec5.shuffle(&mut rnd);
		assert!(okey_check_win(&vec5));

		let mut vec6 = okey_tiles_from_str("r1 r1 r5 r3 r2 b9 b8 b2 b3 y7 y5 y13 k8 k4");
		vec6.shuffle(&mut rnd);
		assert!(!okey_check_win(&vec6));

		let mut vec7 = okey_tiles_from_str("y6 y7 y8 r5 b5 j k5 r8 b8 k8 r13 b13 k13 y13");
		vec7.shuffle(&mut rnd);
		assert!(okey_check_win(&vec7));

		let mut vec = okey_tiles_from_str("y1 y2 y3 y9 y10 y11 r1 b1 k1 y1 r12 b12 k12 y12");
		vec.shuffle(&mut rnd);
		assert!(okey_check_win(&vec));

		let mut vec = okey_tiles_from_str("r7 j r9 b5 b6 b7 r3 b3 k3 y3 r11 b11 k11 y11");
		vec.shuffle(&mut rnd);
		assert!(okey_check_win(&vec));
	}

	#[test_log::test]
	fn test_okey_check_win_fixme() {
		let vec = okey_tiles_from_str("b1 b2 b3 r1 k1 y1 r3 k3 y3 r13 b13 k13 j y13");
		assert!(okey_check_win(&vec), "shit happens");
	}

	#[test]
	fn test_okey_find_most_cost_14_tiles() {
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

		let history_record: Duration = Duration::from_millis(100);

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
	}

	#[test]
	fn test_okey_win_arrange() {
		let mut rng = rand::thread_rng();
		let mut vec = okey_tiles_from_str("j j b7 b7 k8 k8 r1 r1 r2 r2 y11 y11 k13 k13");
		vec.shuffle(&mut rng);

		// let result = okey_arrange_win(&vec);
		// println!("Result: {:?}", result);

		let mut vec = okey_tiles_from_str("y6 y7 y8 r5 b5 j k5 r8 b8 k8 r13 b13 k13 y13");
		vec.shuffle(&mut rng);
		let result = okey_arrange_win(&vec);
		println!("Result: {:?}", result);
	}
}
