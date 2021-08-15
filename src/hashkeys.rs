// hashkeys.rs
use crate::defs;

pub fn generate_pos_key(pos: *const defs::SBoard) -> u64 {
    let mut sq: i32 = 0i32;
    let mut final_key: u64 = 0u64;
    let mut piece: i32;
    while sq < (defs::BRD_SQ_NUM as i32) {
        unsafe {
            piece = (*pos).pieces[sq as usize];
        }
        if (piece != (defs::Squares::NoSq as i32)) && (piece != (defs::Pieces::EMPTY as i32)) && (piece != (defs::Squares::Offboard as i32)) {
            assert!((piece >= (defs::Pieces::WP as i32)) && (piece <= (defs::Pieces::BK as i32)));
            unsafe {
                final_key ^= defs::PIECE_KEYS[piece as usize][sq as usize];
            }
        }
        sq += 1;
    }
    unsafe {
        if (*pos).side == (defs::Colors::White as i32) {
            final_key ^= defs::SIDE_KEY;
        }
        if (*pos).enpas != (defs::Squares::NoSq as i32) {
            assert!(((*pos).enpas >= 0i32) && ((*pos).enpas < defs::BRD_SQ_NUM));
            final_key ^= defs::PIECE_KEYS[defs::Pieces::EMPTY as usize][(*pos).enpas as usize];
        }
        assert!(((*pos).castle_perm >= 0i32) && ((*pos).castle_perm <= 15i32));
        final_key ^= defs::CASTLE_KEYS[(*pos).castle_perm as usize];
        final_key
    }
}