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
        h: usize, w: usize,
        c: [Chars; h],
    }

    let c = add_guard(c, '.');
    let n = h.min(w);
    let mut ans = vec![0; n];

    for (x, y) in iproduct!((1..=h), (1..=w)) {
        if c[x][y] == '#' {
            let arms = UDIR4
                .iter()
                .map(|&(dx, dy)| check(x, y, dx, dy, &c))
                .collect::<HashSet<_>>();
            if arms.len() == 1 {
                ans[arms.iter().next().unwrap() - 2] += 1;
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

fn check(x: usize, y: usize, dx: usize, dy: usize, c: &[Vec<char>]) -> usize {
    if c[x][y] == '.' {
        return 0;
    }

    check(x.wrapping_add(dx), y.wrapping_add(dy), dx, dy, c) + 1
}

const UDIR4: [(usize, usize); 4] = [(!0, !0), (!0, 1), (1, !0), (1, 1)];

fn add_guard<T: Clone>(field: Vec<Vec<T>>, guard: T) -> Vec<Vec<T>> {
    assert!(!field.is_empty());
    assert!(!field[0].is_empty() && field.iter().map(|row| row.len()).all_equal());

    use std::iter::once;
    let width = field[0].len();

    once(vec![guard.clone(); width + 2])
        .chain(field.into_iter().map(|row| {
            once(guard.clone())
                .chain(row)
                .chain(once(guard.clone()))
                .collect()
        }))
        .chain(once(vec![guard.clone(); width + 2]))
        .collect()
}
