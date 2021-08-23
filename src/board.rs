// board.rs
use crate::defs;
use crate::hashkeys;

pub fn check_board(pos: *const defs::SBoard) -> i32 {
    let mut t_pcenum: [i32; 13] = [0; 13];
    let mut t_big_pce: [i32; 2] = [0; 2];
    let mut t_maj_pce: [i32; 2] = [0; 2];
    let mut t_min_pce: [i32; 2] = [0; 2];
    let mut t_material: [i32; 2] = [0; 2];
    let mut sq64: i32;
    let mut t_piece: i32;
    let mut t_pce_num: i32;
    let mut sq120: i32;
    let mut color: i32;
    let mut pcount: i32;
    let mut t_pawns: [u64; 3] = [0u64; 3];
    unsafe {
        t_pawns[(defs::Colors::White as i32) as usize] = (*pos).pawns[(defs::Colors::White as i32) as usize];
        t_pawns[(defs::Colors::Black as i32) as usize] = (*pos).pawns[(defs::Colors::Black as i32) as usize];
        t_pawns[(defs::Colors::Both as i32) as usize] = (*pos).pawns[(defs::Colors::Both as i32) as usize];
    }
    t_piece = defs::Pieces::WP as i32;
    while t_piece <= (defs::Pieces::BK as i32) {
        t_pce_num = 0;
        unsafe {
            while t_pce_num < (*pos).pce_num[t_piece as usize] {
                sq120 = (*pos).p_list[t_piece as usize][t_pce_num as usize];
                assert!((*pos).pieces[sq120 as usize] == t_piece);
                t_pce_num += 1;
            }
        }
        t_piece += 1;
    }
    sq64 = 0;
    while sq64 < 64i32 {
        sq120 = defs::sq120(sq64 as usize);
        unsafe { t_piece = (*pos).pieces[sq120 as usize]; }
        t_pcenum[t_piece as usize] += 1;
        unsafe { 
            color = defs::PIECE_COL[t_piece as usize];
            if defs::PIECE_BIG[t_piece as usize] == (defs::TF::True as i32) {
                t_big_pce[color as usize] += 1;
            }
            if defs::PIECE_MIN[t_piece as usize] == (defs::TF::True as i32) {
                t_min_pce[color as usize] += 1;
            }
            if defs::PIECE_MAJ[t_piece as usize] == (defs::TF::True as i32) {
                t_maj_pce[color as usize] += 1;
            }
            if color != (defs::Colors::Both as i32) {
                t_material[color as usize] += defs::PIECE_VAL[t_piece as usize];
            }
            sq64 += 1;
        }
    }
    t_piece = defs::Pieces::WP as i32;
    while t_piece <= (defs::Pieces::BK as i32) {
        unsafe { assert!(t_pcenum[t_piece as usize] == (*pos).pce_num[t_piece as usize]); }
        t_piece += 1;
    }
    pcount = defs::cnt(t_pawns[(defs::Colors::White as i32) as usize]);
    unsafe { assert!(pcount == (*pos).pce_num[(defs::Pieces::WP as i32) as usize]); }
    pcount = defs::cnt(t_pawns[(defs::Colors::Black as i32) as usize]);
    unsafe { assert!(pcount == (*pos).pce_num[(defs::Pieces::BP as i32) as usize]); }
    pcount = defs::cnt(t_pawns[(defs::Colors::Both as i32) as usize]);
    unsafe { assert!(pcount == ((*pos).pce_num[(defs::Pieces::WP as i32) as usize] + (*pos).pce_num[(defs::Pieces::BP as i32) as usize])); }
    while t_pawns[defs::Colors::White as i32 as usize] > 0u64 {
        sq64 = defs::pop(&mut t_pawns[defs::Colors::White as i32 as usize]);
        unsafe { assert!((*pos).pieces[defs::sq120(sq64 as usize) as usize] == (defs::Pieces::WP as i32)); }
    }
    while t_pawns[defs::Colors::Black as i32 as usize] > 0u64 {
        sq64 = defs::pop(&mut t_pawns[defs::Colors::Black as i32 as usize]);
        unsafe { assert!((*pos).pieces[defs::sq120(sq64 as usize) as usize] == (defs::Pieces::BP as i32)); }
    }
    while t_pawns[defs::Colors::Both as i32 as usize] > 0u64 {
        sq64 = defs::pop(&mut t_pawns[defs::Colors::Both as i32 as usize]);
        unsafe { assert!(((*pos).pieces[defs::sq120(sq64 as usize) as usize] == (defs::Pieces::BP as i32)) || ((*pos).pieces[defs::sq120(sq64 as usize) as usize] == (defs::Pieces::WP as i32))); }
    }
    unsafe {
        assert!((t_material[defs::Colors::White as i32 as usize] == (*pos).material[defs::Colors::White as i32 as usize]) && (t_material[defs::Colors::Black as i32 as usize] == (*pos).material[defs::Colors::Black as i32 as usize]));
        assert!((t_min_pce[defs::Colors::White as i32 as usize] == (*pos).min_pce[defs::Colors::White as i32 as usize]) && (t_min_pce[defs::Colors::Black as i32 as usize] == (*pos).min_pce[defs::Colors::Black as i32 as usize]));
        assert!((t_maj_pce[defs::Colors::White as i32 as usize] == (*pos).maj_pce[defs::Colors::White as i32 as usize]) && (t_maj_pce[defs::Colors::Black as i32 as usize] == (*pos).maj_pce[defs::Colors::Black as i32 as usize]));
        assert!((t_big_pce[defs::Colors::White as i32 as usize] == (*pos).big_pce[defs::Colors::White as i32 as usize]) && (t_big_pce[defs::Colors::Black as i32 as usize] == (*pos).big_pce[defs::Colors::Black as i32 as usize]));
        assert!(((*pos).side == (defs::Colors::White as i32)) || ((*pos).side == (defs::Colors::Black as i32)));
        assert!(hashkeys::generate_pos_key(pos) == (*pos).pos_key);
        assert!(((*pos).enpas == (defs::Squares::NoSq as i32)) || ((defs::RANKS_BRD[(*pos).enpas as usize] == (defs::Rank::Rank6 as i32)) && ((*pos).side == (defs::Colors::White as i32))) 
            || ((defs::RANKS_BRD[(*pos).enpas as usize] == (defs::Rank::Rank3 as i32)) && ((*pos).side == (defs::Colors::Black as i32))));
        assert!((*pos).pieces[(*pos).king_sq[defs::Colors::White as i32 as usize] as usize] == (defs::Pieces::WK as i32));
        assert!((*pos).pieces[(*pos).king_sq[defs::Colors::Black as i32 as usize] as usize] == (defs::Pieces::BK as i32));
    }
    defs::TF::True as i32
}

pub fn update_list_material(pos: *mut defs::SBoard) {
    let mut piece: i32;
    let mut sq: i32;
    let mut index: i32;
    let mut color: i32;
    index = 0i32;
    while index < 120i32 {
        sq = index;
        unsafe { piece = (*pos).pieces[index as usize]; }
        if (piece != (defs::Squares::Offboard as i32)) && (piece != (defs::Pieces::EMPTY as i32)) {
            unsafe {
                color = defs::PIECE_COL[piece as usize];
                if defs::PIECE_BIG[piece as usize] == (defs::TF::True as i32) {
                    (*pos).big_pce[color as usize] += 1;
                }
                if defs::PIECE_MIN[piece as usize] == (defs::TF::True as i32) {
                    (*pos).min_pce[color as usize] += 1;
                }
                if defs::PIECE_MAJ[piece as usize] == (defs::TF::True as i32) {
                    (*pos).maj_pce[color as usize] += 1;
                }
                (*pos).material[color as usize] += defs::PIECE_VAL[piece as usize];
                (*pos).p_list[piece as usize][(*pos).pce_num[piece as usize] as usize] = sq;
                (*pos).pce_num[piece as usize] += 1;
                if piece == (defs::Pieces::WK as i32) {
                    (*pos).king_sq[defs::Colors::White as usize] = sq;
                }
                if piece == (defs::Pieces::BK as i32) {
                    (*pos).king_sq[defs::Colors::Black as usize] = sq;
                }
                if piece == (defs::Pieces::WP as i32) {
                    defs::setbit(&mut (*pos).pawns[defs::Colors::White as usize], defs::sq64(sq as usize));
                    defs::setbit(&mut (*pos).pawns[defs::Colors::Both as usize], defs::sq64(sq as usize));
                }
                else if piece == (defs::Pieces::BP as i32) {
                    defs::setbit(&mut (*pos).pawns[defs::Colors::Black as usize], defs::sq64(sq as usize));
                    defs::setbit(&mut (*pos).pawns[defs::Colors::Both as usize], defs::sq64(sq as usize));
                }
            }
        }
        index += 1;
    }
}

pub fn parse_fen(fen: &str, pos: *mut defs::SBoard) -> i32 {
    //assert!(fen.is_empty() != true);
    let mut rank: i32 = defs::Rank::Rank8 as i32;
    let mut file: i32 = defs::File::FileA as i32;
    let mut piece: i32;
    let mut count: i32;
    let mut i: i32;
    let mut sq64: i32;
    let mut sq120: i32;
    reset_board(pos);
    let mut chars = fen.chars();
    let mut curr_char = chars.next();
    while (rank >= (defs::Rank::Rank1 as i32)) && (curr_char != None) {
        count = 1i32;
        match curr_char {
            Some('p') => { piece = defs::Pieces::BP as i32 }
            Some('r') => { piece = defs::Pieces::BR as i32 }
            Some('n') => { piece = defs::Pieces::BN as i32 }
            Some('b') => { piece = defs::Pieces::BB as i32 }
            Some('k') => { piece = defs::Pieces::BK as i32 }
            Some('q') => { piece = defs::Pieces::BQ as i32 }
            Some('P') => { piece = defs::Pieces::WP as i32 }
            Some('R') => { piece = defs::Pieces::WR as i32 }
            Some('N') => { piece = defs::Pieces::WN as i32 }
            Some('B') => { piece = defs::Pieces::WB as i32 }
            Some('K') => { piece = defs::Pieces::WK as i32 }
            Some('Q') => { piece = defs::Pieces::WQ as i32 }
            Some('1') | Some('2') | Some('3') | Some('4') |
            Some('5') | Some('6') | Some('7') | Some('8') => { 
                piece = defs::Pieces::EMPTY as i32;
                count = (curr_char.unwrap().to_digit(10).unwrap() as i32) - 0i32;
            }
            Some('/') | Some(' ') => {
                rank -= 1i32;
                file = defs::File::FileA as i32;
                curr_char = chars.next();
                continue;
            }
            _ => {
                println!("FEN error");
                return -1
            }
        }
        i = 0;
        while i < count {
            sq64 = rank * 8 as i32 + file;
            sq120 = defs::sq120(sq64 as usize);
            if piece != (defs::Pieces::EMPTY as i32) {
                unsafe {
                    (*pos).pieces[sq120 as usize] = piece;
                }
            }
            file += 1i32;
            i += 1i32;
        }
        curr_char = chars.next();
    }
    assert!(curr_char == Some('w') || curr_char == Some('b'));
    if curr_char == Some('w') {
        unsafe { (*pos).side = defs::Colors::White as i32; }
    }
    else {
        unsafe { (*pos).side = defs::Colors::Black as i32; }
    }
    chars.next();
    curr_char = chars.next();
    i = 0;
    while i < 4i32 {
        if curr_char == Some(' ') {
            break;
        }
        match curr_char {
            Some('K') => { unsafe { (*pos).castle_perm |= defs::CastlePerm::WKCA as i32 } }
            Some('Q') => { unsafe { (*pos).castle_perm |= defs::CastlePerm::WQCA as i32 } }
            Some('k') => { unsafe { (*pos).castle_perm |= defs::CastlePerm::BKCA as i32 } }
            Some('q') => { unsafe { (*pos).castle_perm |= defs::CastlePerm::BQCA as i32 } }
            _ => { }
        }
        curr_char = chars.next();
        i += 1;
    }
    curr_char = chars.next();
    assert!(unsafe{ ((*pos).castle_perm >= 0i32) && ((*pos).castle_perm <= 15i32) });
    if curr_char != Some('-')  {
        match curr_char {
            Some('a') => { file = 0i32; }
            Some('b') => { file = 1i32; }
            Some('c') => { file = 2i32; }
            Some('d') => { file = 3i32; }
            Some('e') => { file = 4i32; }
            Some('f') => { file = 5i32; }
            Some('g') => { file = 6i32; }
            Some('h') => { file = 7i32; }
            _ => { }
        }
        curr_char = chars.next();
        rank = (curr_char.unwrap().to_digit(10).unwrap() as i32) - 1i32;
        assert!((file >= (defs::File::FileA as i32)) && (file <= (defs::File::FileH as i32)));
        assert!((rank >= (defs::Rank::Rank1 as i32)) && (rank <= (defs::Rank::Rank8 as i32)));
        unsafe { (*pos).enpas = defs::fr2_sq(file, rank); }
    }
    unsafe { (*pos).pos_key = hashkeys::generate_pos_key(pos); }
    update_list_material(pos);
    0
}

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
    while index < 2i32 {
        unsafe {
            (*pos).big_pce[index as usize] = 0i32;
            (*pos).maj_pce[index as usize] = 0i32;
            (*pos).min_pce[index as usize] = 0i32;
            (*pos).material[index as usize] = 0i32;
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

pub fn print_board(pos: *const defs::SBoard) {
    let mut sq: i32;
    let mut file: i32;
    let mut rank: i32;
    let mut piece: i32;
    println!();
    println!("Game Board:");
    println!();
    rank = defs::Rank::Rank8 as i32;
    while rank >= defs::Rank::Rank1 as i32 {
        print!("{}   ", rank + 1);
        file = defs::File::FileA as i32;
        while file <= defs::File::FileH as i32 {
            sq = defs::fr2_sq(file, rank);
            unsafe { 
                piece = (*pos).pieces[sq as usize]; 
                print!("{:3}", defs::PCE_CHAR.chars().nth(piece as usize).unwrap());
            }
            file += 1;
        }
        println!();
        rank -= 1;
    }
    println!();
    print!("    ");
    file = defs::File::FileA as i32;
    while file <= defs::File::FileH as i32 {
        print!("{:3}", (('a' as i32) + file) as u8 as char);
        file += 1;
    }
    println!();
    unsafe {
        println!("side: {}", defs::SIDE_CHAR.chars().nth((*pos).side as usize).unwrap());
        println!("enPas: {}", (*pos).enpas);
        print!("castle: ");
        if ((*pos).castle_perm & (defs::CastlePerm::WKCA as i32)) > 0 {
            print!("K");
        }
        else {
            print!("-");
        }
        if ((*pos).castle_perm & (defs::CastlePerm::WQCA as i32)) > 0 {
            print!("Q");
        }
        else {
            print!("-");
        }
        if ((*pos).castle_perm & (defs::CastlePerm::BKCA as i32)) > 0 {
            print!("k");
        }
        else {
            print!("-");
        }
        if ((*pos).castle_perm & (defs::CastlePerm::BQCA as i32)) > 0 {
            println!("q");
        }
        else {
            println!("-");
        }
        println!("PosKey: {:X}", (*pos).pos_key);
    }
}