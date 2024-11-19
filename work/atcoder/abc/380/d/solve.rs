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
        s: Chars,
        q: usize,
        k: [usize; q],
    }

    let k_max = *k.iter().max().unwrap();
    let mut borders = vec![0, s.len()];

    while (borders[borders.len() - 1]).saturating_mul(2) < k_max {
        borders.push(borders[borders.len() - 1] * 2);
    }

    let mut ans = vec![];

    for mut ki in k {
        let mut change = false;
        while ki > s.len() {
            let bi = borders.lower_bound(&ki);
            ki -= borders[bi - 1];
            change = !change;
        }
        ans.push(if change {
            changecase(s[ki - 1])
        } else {
            s[ki - 1]
        });
    }

    println!("{}", ans.iter().join(" "));
}

fn changecase(c: char) -> char {
    if c.is_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}
