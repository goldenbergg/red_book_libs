// defs.rs
use crate::bitboards;

pub const NAME: &str = "Red Book 1.0";
pub const BRD_SQ_NUM: i32 = 120;
pub const MAXGAMEMOVES: i32 = 2048;
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub enum Pieces { EMPTY, WP, WN, WB, WR, WQ, WK, BP, BN, BB, BR, BQ, BK }
pub enum File { FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH, FileNone }
pub enum Rank { Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8, RankNone }

pub enum Colors { White, Black, Both }

pub enum Squares {
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8, NoSq, Offboard
}

pub enum TF { False, True }

pub enum CastlePerm { WKCA = 1, WQCA = 2, BKCA = 4, BQCA = 8 }

pub struct SUndo {
    pub pc_move: i32,
    pub castle_perm: i32,
    pub enpas: i32,
    pub fifty_move: i32,
    pub pos_key: u64,
}

pub struct SBoard {
    pub pieces: [i32; BRD_SQ_NUM as usize],
    pub pawns: [u64; 3],
    pub king_sq: [i32; 2],
    pub side: i32,
    pub enpas: i32,
    pub fifty_move: i32,
    pub ply: i32,
    pub his_ply: i32,
    pub castle_perm: i32,
    pub pos_key: u64,
    pub pce_num: [i32; 13],
    pub big_pce: [i32; 2],
    pub maj_pce: [i32; 2],
    pub min_pce: [i32; 2],
    pub material: [i32; 2],
    //pub history: [SUndo; MAXGAMEMOVES as usize],
    pub p_list: [[i32; 10] ; 13],
}

pub fn fr2_sq(f: i32, r: i32) -> i32 {
    (21 + f) + (r * 10)
}

pub fn sq64(sq120: usize) -> i32 {
    unsafe {
        SQ120_TO_SQ64[sq120]
    }
}

pub fn sq120(sq64: usize) -> i32 {
    unsafe {
        SQ64_TO_SQ120[sq64]
    }
}

pub fn pop(b: *mut u64) -> i32 {
    bitboards::pop_bit(b)
}

pub fn cnt(b: u64) -> i32 {
    bitboards::count_bits(b)
}

pub fn clrbit(bb: *mut u64, sq: i32) {
    unsafe {
        *bb &= CLEAR_MASK[(sq as usize)];
    }
}

pub fn setbit(bb: *mut u64, sq: i32) {
    unsafe {
        *bb |= SET_MASK[(sq as usize)];
    }
}

pub fn is_bq(p: usize) -> i32 {
    unsafe {
        PIECE_BISHOP_QUEEN[p]
    }
}

pub fn is_rq(p: usize) -> i32 {
    unsafe {
        PIECE_ROOK_QUEEN[p]
    }
}

pub fn is_kn(p: usize) -> i32 {
    unsafe {
        PIECE_KNIGHT[p]
    }
}

pub fn is_ki(p: usize) -> i32 {
    unsafe {
        PIECE_KING[p]
    }
}

pub static mut SQ120_TO_SQ64: [i32; BRD_SQ_NUM as usize] = [0; BRD_SQ_NUM as usize];
pub static mut SQ64_TO_SQ120: [i32; 64] = [0; 64];
pub static mut SET_MASK: [u64; 64] = [0u64; 64];
pub static mut CLEAR_MASK: [u64; 64] = [0u64; 64];
pub static mut PIECE_KEYS: [[u64; 120] ; 13] = [[0u64; 120]; 13];
pub static mut SIDE_KEY: u64 = 0;
pub static mut CASTLE_KEYS: [u64; 16] = [0u64; 16];

pub static mut PCE_CHAR: &str = ".PNBRQKpnbrqk";
pub static mut SIDE_CHAR: &str = "wb-";
pub static mut RANK_CHAR: &str = "12345678";
pub static mut FILE_CHAR: &str = "abcdefgh";

pub static mut PIECE_BIG: [i32; 13] = [ 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1 ];
pub static mut PIECE_MAJ: [i32; 13] = [ 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1 ];
pub static mut PIECE_MIN: [i32; 13] = [ 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0 ];
pub static mut PIECE_VAL: [i32; 13] = [ 0, 100, 325, 325, 550, 1000, 50000, 100, 325, 325, 550, 1000, 50000 ];
pub static mut PIECE_COL: [i32; 13] = [ 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1 ];

pub static mut FILES_BRD: [i32; BRD_SQ_NUM as usize] = [0; BRD_SQ_NUM as usize];
pub static mut RANKS_BRD: [i32; BRD_SQ_NUM as usize] = [0; BRD_SQ_NUM as usize];

pub static mut PIECE_KNIGHT: [i32; 13] = [ 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0 ];
pub static mut PIECE_KING: [i32; 13] = [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1 ];
pub static mut PIECE_ROOK_QUEEN: [i32; 13] = [ 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0 ];
pub static mut PIECE_BISHOP_QUEEN: [i32; 13] = [ 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0 ];