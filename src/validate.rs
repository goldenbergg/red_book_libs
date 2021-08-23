// validate.rs
use crate::defs;

pub fn sq_on_board(sq: i32) -> i32 {
    unsafe {
        if defs::FILES_BRD[sq as usize] == (defs::Squares::Offboard as i32) {
            return 0;
        }
        else {
            return 1;
        }
    }
}

pub fn side_valid(side: i32) -> i32 {
    if (side == (defs::Colors::White as i32)) || (side == (defs::Colors::Black as i32)) {
        return 1;
    }
    else {
        return 0;
    }
}

pub fn file_rank_valid(fr: i32) -> i32 {
    if (fr >= 0) && (fr <= 7) {
        return 1;
    }
    else {
        return 0;
    }
}

pub fn piece_valid_empty(pce: i32) -> i32 {
    if (pce >= (defs::Pieces::EMPTY as i32)) && (pce <= (defs::Pieces::BK as i32)) {
        return 1;
    }
    else {
        return 0;
    }
}

pub fn piece_valid(pce: i32) -> i32 {
    if (pce >= (defs::Pieces::WP as i32)) && (pce <= (defs::Pieces::BK as i32)) {
        return 1;
    }
    else {
        return 0;
    }
}