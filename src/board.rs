// board.rs
use crate::defs;

pub fn reset_board(pos: *mut defs::SBoard) {
    let mut index: i32 = 0i32;
    while index < defs::BRD_SQ_NUM {
        unsafe {
            (*pos).pieces[index as usize] = defs::Squares::Offboard as i32;
        }
        index += 1;
    }
    index = 0i32;
    while index < 64i32 {
        unsafe {
            (*pos).pieces[defs::sq120(index as usize) as usize] = defs::Pieces::EMPTY as i32;
        }
        index += 1;
    }
    index = 0i32;
    while index < 3i32 {
        unsafe {
            (*pos).big_pce[index as usize] = 0i32;
            (*pos).maj_pce[index as usize] = 0i32;
            (*pos).min_pce[index as usize] = 0i32;
            (*pos).pawns[index as usize] = 0u64;
        }
        index += 1;
    }
    index = 0i32;
    while index < 13i32 {
        unsafe {
            (*pos).pce_num[index as usize] = 0i32;
        }
        index += 1;
    }
    unsafe {
        (*pos).king_sq[defs::Colors::White as usize] = defs::Squares::NoSq as i32;
        (*pos).king_sq[defs::Colors::Black as usize] = defs::Squares::NoSq as i32;
        (*pos).side = defs::Colors::Both as i32;
        (*pos).enpas = defs::Squares::NoSq as i32;
        (*pos).fifty_move = 0i32;
        (*pos).ply = 0i32;
        (*pos).his_ply = 0i32;
        (*pos).castle_perm = 0i32;
        (*pos).pos_key = 0u64;
    }
}