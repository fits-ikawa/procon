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
        q: usize,
    }

    use ac_library::ModInt998244353 as Mint;

    let mut s = VecDeque::new();
    s.push_back(1);
    let mut cur = Mint::new(1);

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                }
                s.push_back(x);
                cur = cur * 10 + x;
            }
            2 => {
                let d = s.pop_front().unwrap();
                cur = cur - Mint::new(10).pow(s.len() as u64) * d;
            }
            3 => {
                println!("{}", cur);
            }
            _ => unreachable!(),
        }
    }
}
