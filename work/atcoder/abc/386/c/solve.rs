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
        _k: usize,
        s: Chars,
        t: Chars,
    }

    let (s, t) = if s.len() <= t.len() { (s, t) } else { (t, s) };

    let mut cnt_f = 0;
    let mut cnt_b = 0;

    for (&si, &ti) in izip!(s.iter(), t.iter()) {
        if si == ti {
            cnt_f += 1;
        } else {
            break;
        }
    }

    for (&si, &ti) in izip!(s.iter().rev(), t.iter().rev()) {
        if si == ti {
            cnt_b += 1;
        } else {
            break;
        }
    }

    if cnt_f == cnt_b && s.len() == t.len() && cnt_f == s.len() {
        println!("Yes");
    } else if cnt_f + cnt_b >= s.len() && s.len() == t.len() - 1 {
        println!("Yes");
    } else if cnt_f + cnt_b >= s.len() - 1 && s.len() - 1 == t.len() {
        println!("Yes");
    } else if cnt_f + cnt_b == s.len() - 1 && s.len() - 1 == t.len() - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
