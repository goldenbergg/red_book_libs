// init.rs
use crate::defs;

pub fn init_bit_masks() {
	let mut index: i32 = 0 as i32;
	while index < 64 as i32 {
		unsafe {
			defs::SET_MASK[index as usize] = 0 as u64;
			defs::CLEAR_MASK[index as usize] = 0 as u64;
		}
		index += 1;
	}
	index = 0 as i32;
	while index < 64 as i32 {
		unsafe {
			defs::SET_MASK[index as usize] |= (1 as u64) << index;
			defs::CLEAR_MASK[index as usize] = !(defs::SET_MASK[index as usize]);
		}
		index += 1;
	}
}

pub fn init_sq120_to64() {
	let mut sq: i32;
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
	init_bit_masks();
}