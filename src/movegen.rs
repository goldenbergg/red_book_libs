// movegen.rs
use crate::attack;
use crate::board;
use crate::defs;
use crate::validate;

pub fn pc_move(f: i32, t: i32, ca: i32, pro: i32, fl: i32) -> i32 {
    f | (t << 7) | (ca << 14) | (pro << 20) | fl
}

pub fn sq_off_board(sq: i32) -> i32 {
    unsafe {
        if defs::FILES_BRD[sq as usize] == (defs::Squares::Offboard as i32) {
            return 1;
        }
        else {
            return 0;
        }
    }
}

pub const LOOP_SLIDE_PCE: [i32; 8] = [ defs::Pieces::WB as i32, defs::Pieces::WR as i32, defs::Pieces::WQ as i32, 0, 
                                       defs::Pieces::BB as i32, defs::Pieces::BR as i32, defs::Pieces::BQ as i32, 0 ];
pub const LOOP_NON_SLIDE_PCE: [i32; 6] = [ defs::Pieces::WN as i32, defs::Pieces::WK as i32, 0, 
                                           defs::Pieces::BN as i32, defs::Pieces::BK as i32, 0 ];
pub const LOOP_SLIDE_INDEX: [i32; 2] = [ 0, 4 ];
pub const LOOP_NON_SLIDE_INDEX: [i32; 2] = [0, 3];

pub const PCE_DIR: [[i32; 8] ; 13] = [[ 0, 0, 0, 0, 0, 0, 0, 0 ],
                                      [ 0, 0, 0, 0, 0, 0, 0, 0 ],
                                      [ -8, -19, -21, -12, 8, 19, 21, 12 ],
                                      [ -9, -11, 11, 9, 0, 0, 0, 0 ],
                                      [ -1, -10, 1, 10, 0, 0, 0, 0 ],
                                      [ -1, -10, 1, 10, -9, -11, 11, 9 ],
                                      [ -1, -10, 1, 10, -9, -11, 11, 9 ],
                                      [ 0, 0, 0, 0, 0, 0, 0, 0 ],
                                      [ -8, -19, -21, -12, 8, 19, 21, 12 ],
                                      [ -9, -11, 11, 9, 0, 0, 0, 0 ],
                                      [ -1, -10, 1, 10, 0, 0, 0, 0 ],
                                      [ -1, -10, 1, 10, -9, -11, 11, 9 ],
                                      [ -1, -10, 1, 10, -9, -11, 11, 9 ]];

pub const NUM_DIR: [i32; 13] = [ 0, 0, 8, 4, 4, 8, 8, 0, 8, 4, 4, 8, 8 ];

pub fn add_quiet_move(_pos: *const defs::SBoard, pc_move: i32, list: *mut defs::SMoveList) {
    unsafe {
        (*list).moves[(*list).count as usize].pc_move = pc_move;
        (*list).moves[(*list).count as usize].score = 0;
        (*list).count += 1;
    }
}

pub fn add_capture_move(_pos: *const defs::SBoard, pc_move: i32, list: *mut defs::SMoveList) {
    unsafe {
        (*list).moves[(*list).count as usize].pc_move = pc_move;
        (*list).moves[(*list).count as usize].score = 0;
        (*list).count += 1;
    }
}

pub fn add_enpassant_move(_pos: *const defs::SBoard, pc_move: i32, list: *mut defs::SMoveList) {
    unsafe {
        (*list).moves[(*list).count as usize].pc_move = pc_move;
        (*list).moves[(*list).count as usize].score = 0;
        (*list).count += 1;
    }
}

pub fn add_white_pawn_cap_move(pos: *const defs::SBoard, from: i32, to: i32, cap: i32, list: *mut defs::SMoveList) {
    assert!(validate::piece_valid_empty(cap) == 1);
    assert!(validate::sq_on_board(from) == 1);
    assert!(validate::sq_on_board(to) == 1);
    unsafe {
        if defs::RANKS_BRD[from as usize] == (defs::Rank::Rank7 as i32) {
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::WQ as i32, 0), list);
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::WR as i32, 0), list);
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::WB as i32, 0), list);
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::WN as i32, 0), list);
        }
        else {
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::EMPTY as i32, 0), list);
        }
    }
}

pub fn add_white_pawn_move(pos: *const defs::SBoard, from: i32, to: i32, list: *mut defs::SMoveList) {
    assert!(validate::sq_on_board(from) == 1);
    assert!(validate::sq_on_board(to) == 1);
    unsafe {
        if defs::RANKS_BRD[from as usize] == (defs::Rank::Rank7 as i32) {
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::WQ as i32, 0), list);
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::WR as i32, 0), list);
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::WB as i32, 0), list);
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::WN as i32, 0), list);
        }
        else {
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, 0), list);
        }
    }
}

pub fn add_black_pawn_cap_move(pos: *const defs::SBoard, from: i32, to: i32, cap: i32, list: *mut defs::SMoveList) {
    assert!(validate::piece_valid_empty(cap) == 1);
    assert!(validate::sq_on_board(from) == 1);
    assert!(validate::sq_on_board(to) == 1);
    unsafe {
        if defs::RANKS_BRD[from as usize] == (defs::Rank::Rank2 as i32) {
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::BQ as i32, 0), list);
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::BR as i32, 0), list);
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::BB as i32, 0), list);
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::BN as i32, 0), list);
        }
        else {
            add_capture_move(pos, pc_move(from, to, cap, defs::Pieces::EMPTY as i32, 0), list);
        }
    }
}

pub fn add_black_pawn_move(pos: *const defs::SBoard, from: i32, to: i32, list: *mut defs::SMoveList) {
    assert!(validate::sq_on_board(from) == 1);
    assert!(validate::sq_on_board(to) == 1);
    unsafe {
        if defs::RANKS_BRD[from as usize] == (defs::Rank::Rank2 as i32) {
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::BQ as i32, 0), list);
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::BR as i32, 0), list);
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::BB as i32, 0), list);
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::BN as i32, 0), list);
        }
        else {
            add_quiet_move(pos, pc_move(from, to, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, 0), list);
        }
    }
}

pub fn generate_all_moves(pos: *const defs::SBoard, list: *mut defs::SMoveList) {
    unsafe {
        assert!(board::check_board(pos) == 1);
        (*list).count = 0;
        let mut pce: i32;
        let side: i32 = (*pos).side;
        let mut sq: i32;
        let mut t_sq: i32;
        let mut pce_num: i32;
        let mut dir: i32;
        let mut index: i32;
        let mut pce_index: i32;
        println!();
        println!();
        println!("Side: {}", side);
        if side == (defs::Colors::White as i32) {
            pce_num = 0i32;
            while pce_num < (*pos).pce_num[defs::Pieces::WP as i32 as usize] {
                sq = (*pos).p_list[defs::Pieces::WP as i32 as usize][pce_num as usize];
                assert!(validate::sq_on_board(sq) == 1);
                if (*pos).pieces[(sq + 10) as i32 as usize] == (defs::Pieces::EMPTY as i32) {
                    add_white_pawn_move(pos, sq, (sq + 10) as i32, list);
                    if (defs::RANKS_BRD[sq as usize] == (defs::Rank::Rank2 as i32)) && ((*pos).pieces[(sq + 20) as i32 as usize] == (defs::Pieces::EMPTY as i32)) {
                        add_quiet_move(pos, pc_move(sq, (sq + 20) as i32, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGPS), list);
                    }
                }
                if !(sq_off_board(sq + 9) == 1) && (defs::PIECE_COL[((*pos).pieces[(sq + 9) as usize]) as usize] == defs::Colors::Black as i32) {
                    add_white_pawn_cap_move(pos, sq, sq + 9 as i32, (*pos).pieces[(sq + 9) as i32 as usize], list);
                }
                if !(sq_off_board(sq + 11) == 1) && (defs::PIECE_COL[((*pos).pieces[(sq + 11) as usize]) as usize] == defs::Colors::Black as i32) {
                    add_white_pawn_cap_move(pos, sq, sq + 11, (*pos).pieces[(sq + 11) as usize], list);
                }
                if (sq + 9) == (*pos).enpas {
                    add_capture_move(pos, pc_move(sq, sq + 9, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGEP), list);
                }
                if (sq + 11) == (*pos).enpas {
                    add_capture_move(pos, pc_move(sq, sq + 11, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGEP), list);
                }
                pce_num += 1;
            }
            if ((*pos).castle_perm & (defs::CastlePerm::WKCA as i32)) > 0 {
                if ((*pos).pieces[defs::Squares::F1 as i32 as usize] == (defs::Pieces::EMPTY as i32)) && ((*pos).pieces[defs::Squares::G1 as i32 as usize] == (defs::Pieces::EMPTY as i32)) {
                    if (attack::sq_attacked(defs::Squares::E1 as i32, defs::Colors::Black as i32, pos) == 0) && (attack::sq_attacked(defs::Squares::F1 as i32, defs::Colors::Black as i32, pos) == 0) {
                        //println!("WKCA MoveGen");
                        add_quiet_move(pos, pc_move(defs::Squares::E1 as i32, defs::Squares::G1 as i32, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGCA), list);
                    }
                }
            }
            if ((*pos).castle_perm & (defs::CastlePerm::WQCA as i32)) > 0 {
                if ((*pos).pieces[defs::Squares::D1 as i32 as usize] == (defs::Pieces::EMPTY as i32)) && ((*pos).pieces[defs::Squares::C1 as i32 as usize] == (defs::Pieces::EMPTY as i32)) && ((*pos).pieces[defs::Squares::B1 as i32 as usize] == (defs::Pieces::EMPTY as i32)) {
                    if (attack::sq_attacked(defs::Squares::E1 as i32, defs::Colors::Black as i32, pos) == 0) && (attack::sq_attacked(defs::Squares::D1 as i32, defs::Colors::Black as i32, pos) == 0) {
                        //println!("WQCA MoveGen");
                        add_quiet_move(pos, pc_move(defs::Squares::E1 as i32, defs::Squares::C1 as i32, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGCA), list);
                    }
                }
            }
        }
        else {
            pce_num = 0i32;
            while pce_num < (*pos).pce_num[defs::Pieces::BP as i32 as usize] {
                sq = (*pos).p_list[defs::Pieces::BP as i32 as usize][pce_num as usize];
                assert!(validate::sq_on_board(sq) == 1);
                if (*pos).pieces[(sq - 10) as i32 as usize] == (defs::Pieces::EMPTY as i32) {
                    add_black_pawn_move(pos, sq, (sq - 10) as i32, list);
                    if (defs::RANKS_BRD[sq as usize] == (defs::Rank::Rank7 as i32)) && ((*pos).pieces[(sq - 20) as i32 as usize] == (defs::Pieces::EMPTY as i32)) {
                        add_quiet_move(pos, pc_move(sq, (sq - 20) as i32, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGPS), list);
                    }
                }
                if !(sq_off_board(sq - 9) == 1) && (defs::PIECE_COL[((*pos).pieces[(sq - 9) as usize]) as usize] == defs::Colors::White as i32) {
                    add_black_pawn_cap_move(pos, sq, sq - 9 as i32, (*pos).pieces[(sq - 9) as i32 as usize], list);
                }
                if !(sq_off_board(sq - 11) == 1) && (defs::PIECE_COL[((*pos).pieces[(sq - 11) as usize]) as usize] == defs::Colors::White as i32) {
                    add_black_pawn_cap_move(pos, sq, sq - 11, (*pos).pieces[(sq - 11) as usize], list);
                }
                if (sq - 9) == (*pos).enpas {
                    add_capture_move(pos, pc_move(sq, sq - 9, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGEP), list);
                }
                if (sq - 11) == (*pos).enpas {
                    add_capture_move(pos, pc_move(sq, sq - 11, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGEP), list);
                }
                pce_num += 1;
            }
            if ((*pos).castle_perm & (defs::CastlePerm::BKCA as i32)) > 0 {
                if ((*pos).pieces[defs::Squares::F8 as i32 as usize] == (defs::Pieces::EMPTY as i32)) && ((*pos).pieces[defs::Squares::G8 as i32 as usize] == (defs::Pieces::EMPTY as i32)) {
                    if (attack::sq_attacked(defs::Squares::E8 as i32, defs::Colors::White as i32, pos) == 0) && (attack::sq_attacked(defs::Squares::F8 as i32, defs::Colors::White as i32, pos) == 0) {
                        //println!("BKCA MoveGen");
                        add_quiet_move(pos, pc_move(defs::Squares::E8 as i32, defs::Squares::G8 as i32, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGCA), list);
                    }
                }
            }
            if ((*pos).castle_perm & (defs::CastlePerm::BQCA as i32)) > 0 {
                if ((*pos).pieces[defs::Squares::D8 as i32 as usize] == (defs::Pieces::EMPTY as i32)) && ((*pos).pieces[defs::Squares::C8 as i32 as usize] == (defs::Pieces::EMPTY as i32)) && ((*pos).pieces[defs::Squares::B8 as i32 as usize] == (defs::Pieces::EMPTY as i32)) {
                    if (attack::sq_attacked(defs::Squares::E8 as i32, defs::Colors::White as i32, pos) == 0) && (attack::sq_attacked(defs::Squares::D8 as i32, defs::Colors::White as i32, pos) == 0) {
                        //println!("BQCA MoveGen");
                        add_quiet_move(pos, pc_move(defs::Squares::E8 as i32, defs::Squares::C8 as i32, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, defs::MFLAGCA), list);
                    }
                }
            }
        }
        pce_index = LOOP_SLIDE_INDEX[side as usize];
        let fresh0 = pce_index;
        pce_index += 1;
        pce = LOOP_SLIDE_PCE[fresh0 as usize];
        while pce != 0 {
            assert!(validate::piece_valid(pce) == 1);
            //println!("sliders pce_index: {} pce: {}", pce_index, pce);
            pce_num = 0;
            while pce_num < (*pos).pce_num[pce as usize] {
                sq = (*pos).p_list[pce as usize][pce_num as usize];
                assert!(validate::sq_on_board(sq) == 1);
                //println!("Piece {} on {}", defs::PCE_CHAR.chars().nth(pce as usize).unwrap(), io::pr_sq(sq));
                index = 0;
                while index < NUM_DIR[pce as usize] {
                    dir = PCE_DIR[pce as usize][index as usize];
                    t_sq = sq + dir;
                    while sq_off_board(t_sq) == 0 {
                        if (*pos).pieces[t_sq as usize] != (defs::Pieces::EMPTY as i32) {
                            if (defs::PIECE_COL[(*pos).pieces[t_sq as usize] as usize] == side) as i32 ^ 1 as i32 != 0 {
                                //println!("     Capture on {}", io::pr_sq(t_sq));
                                add_capture_move(pos, pc_move(sq, t_sq, (*pos).pieces[t_sq as usize], defs::Pieces::EMPTY as i32, 0), list);
                            }
                            break;
                        }
                        else {
                            //println!("     Normal on {}", io::pr_sq(t_sq));
                            add_quiet_move(pos, pc_move(sq, t_sq, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, 0), list);
                            t_sq += dir;
                        }
                    }
                    index += 1;
                }
                pce_num += 1;
            }
            let fresh1 = pce_index;
            pce_index += 1;
            pce = LOOP_SLIDE_PCE[fresh1 as usize];
        }
        pce_index = LOOP_NON_SLIDE_INDEX[side as usize];
        let fresh2 = pce_index;
        pce_index += 1;
        pce = LOOP_NON_SLIDE_PCE[fresh2 as usize];
        while pce != 0 {
            assert!(validate::piece_valid(pce) == 1);
            //println!("non sliders pce_index: {} pce: {}", pce_index, pce);
            pce_num = 0;
            while pce_num < (*pos).pce_num[pce as usize] {
                sq = (*pos).p_list[pce as usize][pce_num as usize];
                assert!(validate::sq_on_board(sq) == 1);
                //println!("Piece {} on {}", defs::PCE_CHAR.chars().nth(pce as usize).unwrap(), io::pr_sq(sq));
                index = 0;
                while index < NUM_DIR[pce as usize] {
                    dir = PCE_DIR[pce as usize][index as usize];
                    t_sq = sq + dir;
                    if sq_off_board(t_sq) == 0 {
                        if (*pos).pieces[t_sq as usize] != (defs::Pieces::EMPTY as i32) {
                            if (defs::PIECE_COL[(*pos).pieces[t_sq as usize] as usize] == side) as i32 ^ 1 as i32 != 0 {
                                //println!("     Capture on {}", io::pr_sq(t_sq));
                                add_capture_move(pos, pc_move(sq, t_sq, (*pos).pieces[t_sq as usize], defs::Pieces::EMPTY as i32, 0), list);
                            }
                        }
                        else {
                            //println!("     Normal on {}", io::pr_sq(t_sq));
                            add_quiet_move(pos, pc_move(sq, t_sq, defs::Pieces::EMPTY as i32, defs::Pieces::EMPTY as i32, 0), list);
                        }
                    }
                    index += 1;
                }
                pce_num += 1;
            }
            let fresh3 = pce_index;
            pce_index += 1;
            pce = LOOP_NON_SLIDE_PCE[fresh3 as usize];
        }
    }
}