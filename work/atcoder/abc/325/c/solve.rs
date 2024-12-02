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
        s: [Chars; h],
    }

    let s = add_guard(s, '.');
    let mut seen = vec![vec![false; w + 2]; h + 2];

    let mut ans = 0;

    for i in 1..=h {
        for j in 1..=w {
            if s[i][j] == '#' && !seen[i][j] {
                ans += 1;

                let mut todo = VecDeque::new();
                todo.push_back((i, j));
                seen[i][j] = true;

                while let Some((x, y)) = todo.pop_front() {
                    for (nx, ny) in udir8(x, y) {
                        if s[nx][ny] == '#' && !seen[nx][ny] {
                            seen[nx][ny] = true;
                            todo.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}

const UDIR8: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

fn udir8(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR8.iter().map(move |&(dx, dy)| {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        (nx, ny)
    })
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
