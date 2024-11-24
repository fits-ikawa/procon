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
        s: [Chars; n],
    }

    for x in 0..n {
        for y in 0..n {
            let can_put = if s[x][y] == '#' { 2 } else { 1 };
            let mut win = false;

            win |= check(x, y, 1, 0, can_put, 1, n, &s); // 下に見る
            win |= check(x, y, 0, 1, can_put, 1, n, &s); // 右に見る
            win |= check(x, y, 1, 1, can_put, 1, n, &s); // 右下に見る
            win |= check(x, y, !0, 1, can_put, 1, n, &s); // 右上に見る

            if win {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

#[allow(clippy::too_many_arguments)]
fn check(
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
    can_put: usize,
    count: usize,
    n: usize,
    s: &[Vec<char>],
) -> bool {
    if count == 6 {
        return true;
    }

    let nx = x.wrapping_add(dx);
    let ny = y.wrapping_add(dy);

    if nx >= n || ny >= n {
        return false;
    }

    if s[nx][ny] == '.' {
        if can_put > 0 {
            check(nx, ny, dx, dy, can_put - 1, count + 1, n, s)
        } else {
            false
        }
    } else {
        check(nx, ny, dx, dy, can_put, count + 1, n, s)
    }
}
