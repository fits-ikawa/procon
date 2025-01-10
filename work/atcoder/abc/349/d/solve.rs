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
        l: usize, r: usize,
    }

    let mut ans = vec![];

    calc(l, r, &mut ans);
    ans.sort();

    println!("{}", ans.len());

    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn calc(l: usize, r: usize, ans: &mut Vec<(usize, usize)>) {
    if l == r {
        return;
    }

    let exp = (r - l).ilog2();

    for i in (0..=exp).rev() {
        let ti = 2_usize.pow(i);
        for j in l / ti.. {
            if ti * j >= r {
                break;
            }
            if l <= ti * j && ti * (j + 1) <= r {
                ans.push((ti * j, ti * (j + 1)));
                calc(l, ti * j, ans);
                calc(ti * (j + 1), r, ans);
                return;
            }
        }
    }

    unreachable!();
}
