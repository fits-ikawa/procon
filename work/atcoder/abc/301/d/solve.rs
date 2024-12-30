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
        s: Chars,
        n: usize,
    }

    let mut nmin = usize::from_str_radix(
        &s.iter()
            .map(|&si| if si == '?' { '0' } else { si })
            .collect::<String>(),
        2,
    )
    .unwrap();

    for i in 0..s.len() {
        if s[i] == '?' {
            let a = nmin | 1 << s.len() - i - 1;
            if a <= n {
                nmin = a;
            }
        }
    }

    if nmin <= n {
        println!("{}", nmin);
    } else {
        println!("-1");
    }
}
