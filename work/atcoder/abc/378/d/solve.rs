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
        h: usize, w: usize, k: usize,
        field: [Chars; h],
    }

    let field = add_guard(field, '#');

    let mut seen = vec![vec![false; w + 2]; h + 2];
    let mut ans = 0;

    for i in 1..=h {
        for j in 1..=w {
            if field[i][j] == '.' {
                ans += dfs(i, j, k, &field, &mut seen);
            }
        }
    }

    println!("{}", ans);
}

fn dfs(i: usize, j: usize, k: usize, field: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) -> usize {
    if k == 0 {
        return 1;
    }

    seen[i][j] = true;

    let ans = udir4(i, j)
        .map(|(y, x)| {
            if !seen[y][x] && field[y][x] == '.' {
                dfs(y, x, k - 1, field, seen)
            } else {
                0
            }
        })
        .sum::<usize>();

    seen[i][j] = false;

    ans
}

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

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(y: usize, x: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dy, dx)| {
        let new_y = y.wrapping_add(dy);
        let new_x = x.wrapping_add(dx);
        (new_y, new_x)
    })
}
