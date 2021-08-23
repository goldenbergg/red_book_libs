// attack.rs
use crate::defs;
use crate::board;
use crate::validate;

pub const KN_DIR: [i32; 8] = [ -8, -19,	-21, -12, 8, 19, 21, 12 ];
pub const RK_DIR: [i32; 4] = [ -1, -10,	1, 10 ];
pub const BI_DIR: [i32; 4] = [ -9, -11, 11, 9 ];
pub const KI_DIR: [i32; 8] = [ -1, -10,	1, 10, -9, -11, 11, 9 ];

pub fn sq_attacked (sq: i32, side: i32, pos: *const defs::SBoard) -> i32 {
    let mut pce: i32;
    let mut index: i32;
    let mut t_sq: i32;
    let mut dir: i32;
    assert!(validate::sq_on_board(sq) == 1);
    assert!(validate::side_valid(side) == 1);
    assert!(board::check_board(pos) == 1);
    if side == (defs::Colors::White as i32) {
        unsafe {
            if ((*pos).pieces[(sq - 11) as usize] == (defs::Pieces::WP as i32)) || ((*pos).pieces[(sq - 9) as usize] == (defs::Pieces::WP as i32)) {
                return defs::TF::True as i32;
            }
        }
    }
    else {
        unsafe {
            if ((*pos).pieces[(sq + 11) as usize] == (defs::Pieces::BP as i32)) || ((*pos).pieces[(sq + 9) as usize] == (defs::Pieces::BP as i32)) {
                return defs::TF::True as i32;
            }
        }
    }
    index = 0;
    while index < 8 {
        unsafe {
            pce = (*pos).pieces[(sq + KN_DIR[index as usize]) as usize];
            if pce != (defs::Squares::Offboard as i32) {
                if (defs::is_kn(pce as usize) > 0) && (defs::PIECE_COL[pce as usize] == side) {
                    return defs::TF::True as i32;
                }
            }
        }
        index += 1;
    }
    index = 0;
    while index < 4 {
        dir = RK_DIR[index as usize];
        t_sq = sq + dir;
        unsafe { pce = (*pos).pieces[t_sq as usize]; }
        while pce != (defs::Squares::Offboard as i32) {
            if pce != (defs::Pieces::EMPTY as i32) {
                unsafe {
                    if (defs::is_rq(pce as usize) > 0) && (defs::PIECE_COL[pce as usize] == side) {
                        return defs::TF::True as i32;
                    }
                    break;
                }
            }
            t_sq += dir;
            unsafe { pce = (*pos).pieces[t_sq as usize]; }
        }
        index += 1;
    }
    index = 0;
    while index < 4 {
        dir = BI_DIR[index as usize];
        t_sq = sq + dir;
        unsafe { pce = (*pos).pieces[t_sq as usize]; }
        while pce != (defs::Squares::Offboard as i32) {
            if pce != (defs::Pieces::EMPTY as i32) {
                unsafe {
                    if (defs::is_bq(pce as usize) > 0) && (defs::PIECE_COL[pce as usize] == side) {
                        return defs::TF::True as i32;
                    }
                    break;
                }
            }
            t_sq += dir;
            unsafe { pce = (*pos).pieces[t_sq as usize]; }
        }
        index += 1;
    }
    index = 0;
    while index < 8 {
        unsafe {
            pce = (*pos).pieces[(sq + KI_DIR[index as usize]) as usize];
            if pce != (defs::Squares::Offboard as i32) {
                if (defs::is_ki(pce as usize) > 0) && (defs::PIECE_COL[pce as usize] == side) {
                    return defs::TF::True as i32;
                }
            }
        }
        index += 1;
    }
    defs::TF::False as i32
}