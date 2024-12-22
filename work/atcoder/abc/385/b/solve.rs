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
        h: usize, w: usize, mut x: Usize1, mut y: Usize1,
        s: [Chars; h],
        t: Chars,
    }

    let mut cnt = 0;
    let mut house = hashset! {};

    for c in t {
        let (nx, ny) = match c {
            'U' => (x.wrapping_add(!0), y),
            'D' => (x + 1, y),
            'L' => (x, y.wrapping_add(!0)),
            'R' => (x, y + 1),
            _ => unreachable!(),
        };

        if nx < h && ny < w && s[nx][ny] != '#' {
            x = nx;
            y = ny;
            if s[nx][ny] == '@' && !house.contains(&(x, y)) {
                cnt += 1;
                house.insert((x, y));
            }
        }
    }

    println!("{} {} {}", x + 1, y + 1, cnt);
}
