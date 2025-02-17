#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise;
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

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [Chars; h],
    }

    let a = a
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|x| if x == '+' { 1 } else { -1 })
                .collect_vec()
        })
        .collect_vec();

    let result = game(0, 0, h, w, &a);

    println!(
        "{}",
        match result.0.cmp(&result.1) {
            Greater => "Takahashi",
            Less => "Aoki",
            Equal => "Draw",
        }
    );
}

#[memoise(x <= 2000, y <= 2000)]
fn game(x: usize, y: usize, h: usize, w: usize, a: &[Vec<isize>]) -> (isize, isize) {
    let turn = (x + y) % 2 == 0;
    let mut cand = vec![];

    if turn {
        // 高橋君のターン
        if x + 1 < h {
            let p = game(x + 1, y, h, w, a);
            cand.push((p.0 + a[x + 1][y], p.1));
        }
        if y + 1 < w {
            let p = game(x, y + 1, h, w, a);
            cand.push((p.0 + a[x][y + 1], p.1));
        }

        if cand.is_empty() {
            (0, 0)
        } else {
            cand.sort_by_key(|p| p.1 - p.0);
            cand[0]
        }
    } else {
        // 青木君のターン
        if x + 1 < h {
            let p = game(x + 1, y, h, w, a);
            cand.push((p.0, p.1 + a[x + 1][y]));
        }
        if y + 1 < w {
            let p = game(x, y + 1, h, w, a);
            cand.push((p.0, p.1 + a[x][y + 1]));
        }

        if cand.is_empty() {
            (0, 0)
        } else {
            cand.sort_by_key(|p| p.0 - p.1);
            cand[0]
        }
    }
}
