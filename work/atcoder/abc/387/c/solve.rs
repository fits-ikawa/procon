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

    println!("{}", calc(r) - calc(l - 1));
}

fn calc(n: usize) -> usize {
    if n < 10 {
        return 0;
    }

    let mut ret = 0;

    for i in 2..=19 {
        let base = 10_usize.pow(i);
        if base <= n {
            for j in 1..=9_usize {
                ret += j.pow(i - 1);
            }
        } else {
            let left = n / (base / 10);

            for k in 1..left {
                ret += k.pow(i - 1);
            }

            let mut m = n % (base / 10);
            let mut b = base / 100;
            let mut j = i - 1;

            while b > 0 {
                let d = m / b;

                ret += d.min(left) * left.pow(j - 1);

                if d >= left {
                    break;
                }

                m %= b;
                b /= 10;
                j -= 1;
            }

            if j == 0 {
                // n がヘビ数なので +1
                ret += 1;
            }

            break;
        }
    }

    ret
}
