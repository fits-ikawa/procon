#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use std::os::unix::net::SocketAddr;
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
        _n: usize, k: usize,
        a: [Usize1; k],
    }

    if a.len() == 1 {
        println!("{}", 0);
        return;
    }

    let mut acc_fw = a
        .chunks_exact(2)
        .map(|w| w[1] - w[0])
        .cumsum::<usize>()
        .collect_vec();

    let mut acc_bk = a
        .iter()
        .rev()
        .collect_vec()
        .chunks(2)
        .filter_map(|w| {
            if w.len() == 2 {
                Some(w[0] - w[1])
            } else {
                None
            }
        })
        .cumsum::<usize>()
        .collect_vec();

    let n_pairs = acc_fw.len();
    acc_fw.insert(0, 0);
    acc_bk.insert(0, 0);

    if a.len() % 2 == 0 {
        println!("{}", acc_fw.last().unwrap());
        return;
    }

    let mut ans = usize::MAX;

    for i in 0..=n_pairs {
        ans = ans.min(acc_fw[i] + acc_bk[n_pairs - i]);
    }

    println!("{}", ans);
}
