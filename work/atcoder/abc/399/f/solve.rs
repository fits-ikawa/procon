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
        n: usize, k: usize,
        a: [usize; n],
    }

    use ac_library::ModInt998244353 as Mint;

    let acc = std::iter::once(0)
        .chain(a)
        .cumsum::<usize>()
        .map(Mint::new)
        .collect_vec();

    let mut s = vec![vec![Mint::new(1); k + 1]; n + 1];

    for i in 0..=n {
        s[i][1] = acc[i];
    }

    for i in 0..=n {
        for j in 2..=k {
            s[i][j] = s[i][j - 1] * s[i][1];
        }
    }

    let mut ans = Mint::new(0);

    for j in 0..=k {
        let mut x = Mint::new(0);
        let mut y = Mint::new(0);

        for i in 0..n {
            y += s[i][j] * if j % 2 == 0 { 1 } else { -1 };
            x += s[i + 1][k - j] * y;
        }

        ans += x * comb(k, j);
    }

    println!("{}", ans);
}

fn comb(n: usize, r: usize) -> usize {
    debug_assert!(n >= r);
    let r = r.min(n - r);
    let mut result = 1;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}
