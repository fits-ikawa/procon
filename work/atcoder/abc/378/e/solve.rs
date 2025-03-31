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
        n: usize, m: usize,
        a: [usize; n],
    }

    use ac_library::ModInt;
    ModInt::set_modulus(m as u32);

    let s = std::iter::once(0)
        .chain(a)
        .cumsum::<usize>()
        .map(|x| x % m)
        .collect_vec();

    let mut bit = ac_library::FenwickTree::new(m, 0_usize);
    let mut sl_acc = 0;
    let mut ans = 0;

    for r in 0..n {
        sl_acc += s[r];
        ans += s[r + 1] * (r + 1) + m * bit.sum(s[r + 1] + 1..) - sl_acc;
        bit.add(s[r + 1], 1);
    }

    println!("{}", ans);
}
