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

    let mut ans = usize::MAX;

    for target in '0'..='9' {
        let mut t = 0;
        let mut rolling = (0..n).collect::<HashSet<_>>();

        while !rolling.is_empty() {
            let mut next = (usize::MAX, 0);

            for &reel in &rolling {
                let pos = s[reel].iter().position(|&x| x == target).unwrap();
                let t_wrapped = t % 10;
                let time = if pos >= t_wrapped {
                    pos - t_wrapped
                } else {
                    10 - t_wrapped + pos
                };
                next = next.min((time, reel));
            }

            t += next.0 + 1;
            rolling.remove(&next.1);
        }

        ans = ans.min(t - 1);
    }

    println!("{}", ans);
}
