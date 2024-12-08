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
        q: usize,
    }

    let mut tube: VecDeque<(usize, usize)> = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize, c: usize,
                }

                if !tube.is_empty() && tube.back().copied().unwrap().0 == x {
                    let (_, d) = tube.pop_back().unwrap();
                    tube.push_back((x, c + d));
                } else {
                    tube.push_back((x, c));
                }
            }
            2 => {
                input! {
                    mut c: usize,
                }

                let mut ans = 0;

                while c > 0 {
                    let (y, d) = tube.pop_front().unwrap();
                    if d >= c {
                        ans += y * c;
                        tube.push_front((y, d - c));
                        c = 0;
                    } else {
                        ans += y * d;
                        c -= d;
                    }
                }

                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
