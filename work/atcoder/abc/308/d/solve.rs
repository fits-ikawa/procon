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
        h: usize, w: usize,
        s: [Chars; h],
    }

    let next = hashmap! {
        's' => 'n',
        'n' => 'u',
        'u' => 'k',
        'k' => 'e',
        'e' => 's',
    };

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![false; w]; h];

    if s[0][0] == 's' {
        todo.push_back((0, 0, 's'));
        seen[0][0] = true;
    }

    while let Some((x, y, c)) = todo.pop_front() {
        if (x, y) == (h - 1, w - 1) {
            println!("Yes");
            return;
        }
        for (nx, ny) in [
            (x.wrapping_add(!0), y),
            (x, y.wrapping_add(!0)),
            (x + 1, y),
            (x, y + 1),
        ] {
            if nx < h && ny < w && s[nx][ny] == next[&c] && !seen[nx][ny] {
                todo.push_back((nx, ny, next[&c]));
                seen[nx][ny] = true;
            }
        }
    }

    println!("No");
}
