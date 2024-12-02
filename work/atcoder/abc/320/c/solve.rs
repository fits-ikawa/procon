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
        m: usize,
        s: [Chars; 3],
    }

    let mut ans = usize::MAX;

    'outer: for target in '0'..='9' {
        for perm in (0..3).permutations(3) {
            let mut t = 0;

            for reel in perm {
                let t_wrapped = t % m;

                let pos1 = s[reel][t_wrapped..].iter().position(|&x| x == target);
                let pos2 = s[reel][..t_wrapped].iter().position(|&x| x == target);

                let pos = match (pos1, pos2) {
                    (Some(p), _) => Some(t_wrapped + p),
                    (_, Some(p)) => Some(p),
                    (None, None) => None,
                };

                if pos.is_none() {
                    continue 'outer;
                }

                let pos = pos.unwrap();

                let time = if pos >= t_wrapped {
                    pos - t_wrapped
                } else {
                    m - t_wrapped + pos
                };

                t += time + 1
            }

            ans = ans.min(t - 1);
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
