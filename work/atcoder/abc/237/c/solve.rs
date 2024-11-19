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
    }

    let unique = s.iter().copied().collect::<HashSet<_>>();
    if unique == hashset! {'a'} {
        println!("Yes");
        return;
    }

    let s_len = s.len();

    let mut front_a: usize = 0;
    let mut back_a: usize = 0;

    for i in 0..s_len {
        if s[i] == 'a' {
            front_a += 1;
        } else {
            break;
        }
    }
    for i in 0..s_len {
        if s[s_len - 1 - i] == 'a' {
            back_a += 1;
        } else {
            break;
        }
    }

    let mut t = vec!['a'; back_a.saturating_sub(front_a)];
    t.extend(s);

    let t_len = t.len();

    for i in 0..t_len / 2 {
        if t[i] != t[t_len - 1 - i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
