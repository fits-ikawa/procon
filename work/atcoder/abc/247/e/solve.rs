#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize, x: usize, y: usize,
        a: [usize; n],
    }

    let mut bs = vec![];
    let mut cur = vec![];

    for i in 0..n {
        if y <= a[i] && a[i] <= x {
            cur.push(a[i]);
        } else {
            if !cur.is_empty() {
                bs.push(cur);
                cur = vec![];
            }
        }
    }

    if !cur.is_empty() {
        bs.push(cur);
    }

    let mut ans = 0;

    for b in bs {
        // 尺取り法
        let mut right = 0;
        let mut cnt_x = 0;
        let mut cnt_y = 0;

        for left in 0..b.len() {
            while right < b.len() && (cnt_x == 0 || cnt_y == 0) {
                if b[right] == x {
                    cnt_x += 1;
                }
                if b[right] == y {
                    cnt_y += 1;
                }
                right += 1;
            }

            if cnt_x >= 1 && cnt_y >= 1 {
                ans += b.len() + 1 - right;
            }

            if b[left] == x {
                cnt_x -= 1;
            }
            if b[left] == y {
                cnt_y -= 1;
            }
        }
    }

    println!("{}", ans);
}
