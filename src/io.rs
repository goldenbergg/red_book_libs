// io.rs
use crate::defs;

pub fn pr_sq(sq: i32) -> String {
    let mut sq_str = String::new();
    let pc_file: i32;
    let pc_rank: i32;
    unsafe {
        pc_file = defs::FILES_BRD[sq as usize];
        pc_rank = defs::RANKS_BRD[sq as usize];
    }
    sq_str.push((97 + pc_file) as u8 as char);
        sq_str.push((49 + pc_rank) as u8 as char);
        sq_str
}

pub fn pr_move(pc_move: i32) -> String {
    let mut mv_str = String::new();
    let ff: i32;
    let rf: i32;
    let ft: i32;
    let rt: i32;
    unsafe {
        ff = defs::FILES_BRD[defs::from_sq(pc_move) as usize];
        rf = defs::RANKS_BRD[defs::from_sq(pc_move) as usize];
        ft = defs::FILES_BRD[defs::to_sq(pc_move) as usize];
        rt = defs::RANKS_BRD[defs::to_sq(pc_move) as usize];
    }
    let promoted: i32 = defs::promoted(pc_move);
    if promoted > 0 {
        let mut pchar: char = 'q';
        if defs::is_kn(promoted as usize) > 0 {
            pchar = 'n';
        }
        else if (defs::is_rq(promoted as usize) & !(defs::is_bq(promoted as usize))) > 0 {
            pchar = 'r';
        }
        else if (!(defs::is_rq(promoted as usize)) & defs::is_bq(promoted as usize)) > 0 {
            pchar = 'b';
        }
        mv_str.push((97 + ff) as u8 as char);
        mv_str.push((49 + rf) as u8 as char);
        mv_str.push((97 + ft) as u8 as char);
        mv_str.push((49 + rt) as u8 as char);
        mv_str.push(pchar);
    }
    else {
        mv_str.push((97 + ff) as u8 as char);
        mv_str.push((49 + rf) as u8 as char);
        mv_str.push((97 + ft) as u8 as char);
        mv_str.push((49 + rt) as u8 as char);
    }
    mv_str
}

pub fn print_move_list(list: *const defs::SMoveList) {
    let mut index: i32;
    let mut score: i32;
    let mut pc_move: i32;
    unsafe {
        println!("MoveList:");
        index = 0;
        while index < (*list).count {
            pc_move = (*list).moves[index as usize].pc_move;
            score = (*list).moves[index as usize].score;
            println!("Move {} > {} (score: {})", index + 1, pr_move(pc_move), score);
            index += 1;
        }
        println!("MoveList Total {} Moves:", (*list).count);
        println!();
    }
    
}