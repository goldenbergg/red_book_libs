// bitboards.rs
use crate::defs;

pub const BIT_TABLE: [i32; 64] = [
	63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2,
	51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52,
	26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28,
	58, 20, 37, 17, 36, 8
];

pub fn pop_bit(bb: *mut u64) -> i32 {
	unsafe {
		let b: u64 = *bb ^ (*bb).wrapping_sub(1 as i32 as u64);
		let fold: u32 = (b & 0xffffffff as u32 as u64 ^ b >> 32 as i32) as u32;
		*bb &= (*bb).wrapping_sub(1 as i32 as u64);
		BIT_TABLE[(fold.wrapping_mul(0x783a9b23 as i32 as u32) >> 26 as i32) as usize]
	}
}

pub fn count_bits(mut b: u64) -> i32 {
	let mut r: i32 = 0;
	while b > 0 {
		b &= b - 1;
		r += 1;
	}
	r
}

pub fn print_bit_board(bb: u64) {
	let shift_me: u64 = 1u64;
	let mut rank: i32 = defs::Rank::Rank8 as i32;
	let mut file: i32 = defs::File::FileA as i32;
	let mut sq: i32;
	let mut sq64: i32;
	println!();
	while rank >= (defs::Rank::Rank1 as i32) {
		while file <= (defs::File::FileH as i32) {
			sq = defs::fr2_sq(file, rank);
			sq64 = defs::sq64(sq as usize);
			if ((shift_me << sq64) & bb) > 0 {
				print!("X");
			}
			else {
				print!("-");
			}
			file += 1;
		}
		rank -= 1;
		file = defs::File::FileA as i32;
		println!();
	}
	println!();
	println!();
}