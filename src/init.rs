// init.rs
use crate::defs;

pub fn init_sq120_to64() {
	let mut sq: i32 = defs::Squares::A1 as i32;
	let mut sq64: i32 = 0;

	for i in 0..defs::BRD_SQ_NUM {
		unsafe {
			defs::SQ120_TO_SQ64[i as usize] = 65;
		}
	}

	for i in 0..64 {
		unsafe {
			defs::SQ64_TO_SQ120[i as usize] = 120;
		}
	}

	let mut rank: i32 = defs::Rank::Rank1 as i32;
	let mut file: i32 = defs::File::FileA as i32;
	while rank <= (defs::Rank::Rank8 as i32) {
		while file <= (defs::File::FileH as i32) {
			sq = defs::fr2_sq(file, rank);
			unsafe {
				defs::SQ64_TO_SQ120[sq64 as usize] = sq;
				defs::SQ120_TO_SQ64[sq as usize] = sq64;
			}
			sq64 += 1;
			file += 1;
		}
		rank += 1;
		file = defs::File::FileA as i32;
	}
}

pub fn all_init() {
	init_sq120_to64();
}