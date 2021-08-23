// makemove.rs
use crate::defs;

pub fn hash_pce(pos: *mut defs::SBoard, pce: i32, sq: i32) {
    unsafe {
        (*pos).pos_key ^= defs::PIECE_KEYS[pce as usize][sq as usize];
    }
}

pub fn hash_ca(pos: *mut defs::SBoard) {
    unsafe {
        (*pos).pos_key ^= defs::CASTLE_KEYS[(*pos).castle_perm as usize];
    }
}

pub fn hash_side(pos: *mut defs::SBoard) {
    unsafe {
        (*pos).pos_key ^= defs::SIDE_KEY;
    }
}

pub fn hash_ep(pos: *mut defs::SBoard) {
    unsafe {
        (*pos).pos_key ^= defs::PIECE_KEYS[defs::Pieces::EMPTY as i32 as usize][(*pos).enpas as usize];
    }
}

pub const CASTLE_PERM: [i32; 120] = [
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 13, 15, 15, 15, 12, 15, 15, 14, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15,  7, 15, 15, 15,  3, 15, 15, 11, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15
];