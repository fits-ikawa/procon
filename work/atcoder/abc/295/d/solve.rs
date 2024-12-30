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
        s: Chars,
    }

    let mut t = vec![0; s.len() + 1];
    let mut cnt = vec![0; 10];

    for i in 1..s.len() + 1 {
        let d = (s[i - 1] as u8 - b'0') as usize;
        cnt[d] += 1;
        t[i] = t[i - 1] ^ (1 << d);
    }

    let mut ans = 0;

    for &v in t.into_iter().counts().values() {
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}
