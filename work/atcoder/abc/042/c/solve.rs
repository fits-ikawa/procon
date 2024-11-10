#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering, Reverse};
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
fn main() {
    input! {
        n: usize, k: usize,
        d: [usize; k],
    }

    let mut ans = n;

    loop {
        let mut check = ans;
        let mut all_like = true;
        while check > 0 {
            let i = check % 10;
            all_like = all_like && !d.contains(&i);
            check /= 10;
        }
        if all_like {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, k: usize,
        d: [char; k],
    }

    let d = d.into_iter().collect::<HashSet<_>>();

    let mut ans = n;

    loop {
        if d.difference(&ans.to_string().chars().collect::<HashSet<_>>())
            .collect_vec()
            .len()
            == k
        {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}
