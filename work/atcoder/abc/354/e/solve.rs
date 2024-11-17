#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let table = [true; 18];
    let mut memo = hashmap! {};

    println!("{}", 3_usize.pow(9));

    println!(
        "{}",
        if game(true, table, &ab, &mut memo) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

fn game(
    takahashi: bool,
    table: [bool; 18],
    cards: &[(usize, usize)],
    memo: &mut HashMap<(bool, [bool; 18]), bool>,
) -> bool {
    if let Some(&ret) = memo.get(&(takahashi, table)) {
        return ret;
    }

    let mut check = || {
        let mut new_table = table;
        for i in 0..cards.len() - 1 {
            for j in i + 1..cards.len() {
                if table[i] && table[j] && (cards[i].0 == cards[j].0 || cards[i].1 == cards[j].1) {
                    // ペアを取る
                    new_table[i] = false;
                    new_table[j] = false;
                    if !game(!takahashi, new_table, cards, memo) {
                        return true;
                    }
                    // 次のチェックのためにペアを戻す
                    new_table[i] = true;
                    new_table[j] = true;
                }
            }
        }
        // どの取り方でも負け or 取れるペアがない
        false
    };

    let ret = check();
    memo.insert((takahashi, table), ret);
    ret
}
