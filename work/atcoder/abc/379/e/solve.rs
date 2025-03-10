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
        s: Bytes,
    }

    let mut sum = 0;

    for i in 0..n {
        sum += (s[i] - b'0') as usize * (i + 1);
    }

    let mut carry = 0;
    let mut ans = vec![];

    for i in (0..n).rev() {
        ans.push((sum + carry) % 10);
        carry = (sum + carry) / 10;
        sum -= (s[i] - b'0') as usize * (i + 1);
    }

    while carry > 0 {
        ans.push(carry % 10);
        carry /= 10;
    }

    println!("{}", ans.iter().rev().join(""));
}
