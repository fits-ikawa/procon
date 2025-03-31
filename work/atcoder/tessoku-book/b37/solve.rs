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
        n: usize,
    }

    let mut ans = 0;
    let mut base = 1;

    while base <= n {
        ans += f(n, base);
        base *= 10;
    }

    println!("{}", ans);
}

fn f(n: usize, base: usize) -> usize {
    let block = (1..=9).map(|i| i * base).sum::<usize>();

    let mut ret = block * (n / (base * 10));
    let mut m = n % (base * 10) + 1;

    for i in 0..=9 {
        if m >= base {
            ret += i * base;
            m -= base;
        } else {
            ret += i * m;
            break;
        }
    }

    ret
}
