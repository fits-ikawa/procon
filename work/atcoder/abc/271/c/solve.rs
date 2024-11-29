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
        n: usize,
        a: [usize; n],
    }

    use Manga::*;

    let mut sell_q = VecDeque::new();
    let mut count = a.into_iter().counts();
    let sorted = count.keys().sorted().copied().collect_vec();

    for &k in &sorted {
        if count[&k] >= 2 {
            for _ in 0..count[&k] - 1 {
                sell_q.push_back(Dup(k));
            }
        }
    }

    for &k in sorted.iter().rev() {
        sell_q.push_back(Uniq(k));
    }

    let mut ans = 0;

    'outer: for i in 1.. {
        if !count.contains_key(&i) {
            if sell_q.len() >= 2 {
                for _ in 0..2 {
                    if let Uniq(k) = sell_q.pop_front().unwrap() {
                        count.remove(&k);
                        if k < i {
                            break 'outer;
                        }
                    }
                }
            } else {
                break;
            }
        }
        ans = i;
    }

    println!("{}", ans);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Manga {
    Uniq(usize),
    Dup(usize),
}
