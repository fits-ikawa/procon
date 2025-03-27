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
        s: Chars,
    }

    let mut s = s;

    let mut ans = 0;

    if s[0] != 'i' {
        s.insert(0, 'i');
        ans += 1;
    }

    if s.last().copied().unwrap() != 'o' {
        s.push('o');
        ans += 1;
    }

    let cnt = s.iter().dedup_with_count().collect_vec();

    ans += cnt.into_iter().map(|(v, _)| v - 1).sum::<usize>();

    println!("{}", ans);
}
