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
    // セグ木
    input! {
        n: usize,
        p: [Usize1; n],
    }

    use ac_library::{Additive, Segtree};

    let mut seg = Segtree::<Additive<usize>>::new(n);
    for i in 0..n {
        seg.set(i, 1);
    }

    let mut ans = vec![0; n];

    for i in (0..n).rev() {
        let pos = seg.max_right(0, |&x| x <= p[i]);

        ans[pos] = i + 1;
        seg.set(pos, 0);
    }

    println!("{}", ans.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    // フェニック木（二分探索を手実装）
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut bit = ac_library::FenwickTree::new(n, 0_isize);
    for i in 0..n {
        bit.add(i, 1);
    }

    let mut ans = vec![0; n];

    for i in (0..n).rev() {
        let mut left = -1_isize;
        let mut right = n as isize;

        while right - left > 1 {
            let mid = (left + right) / 2;

            if bit.sum(..mid as usize + 1) as usize > p[i] {
                right = mid;
            } else {
                left = mid;
            }
        }

        ans[right as usize] = i + 1;
        bit.add(right as usize, -1);
    }

    println!("{}", ans.iter().join(" "));
}
