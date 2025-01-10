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
        h: usize, w: usize,
        s: [Chars; h],
    }

    use mylib::OptionExt;

    let (mut sx, mut sy) = (0, 0);
    let (mut gx, mut gy) = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                sx = i;
                sy = j;
            } else if s[i][j] == 'G' {
                gx = i;
                gy = j;
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![vec![None; 2]; w]; h];

    todo.push_back((sx, sy, 0)); // 縦移動で sx, sy に入った
    todo.push_back((sx, sy, 1)); // 横移動で sx, sy に入った
    seen[sx][sy][0] = Some(0);
    seen[sx][sy][1] = Some(0);

    while let Some((x, y, dir)) = todo.pop_front() {
        let dirs = if dir == 0 {
            vec![(0, !0), (0, 1)]
        } else {
            vec![(!0, 0), (1, 0)]
        };

        for (dx, dy) in dirs {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            let ndir = if dir == 0 { 1 } else { 0 };

            if nx < h && ny < w && s[nx][ny] != '#' && seen[nx][ny][ndir].is_none() {
                seen[nx][ny][ndir] = seen[x][y][dir].map(|e| e + 1);
                todo.push_back((nx, ny, ndir));
            }
        }
    }

    if let Some(ans) = seen[gx][gy][0].min_or(seen[gx][gy][1]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

pub mod mylib {
    use std::cmp::Ord;

    pub trait OptionExt<T> {
        /// Returns the maximum of two `Option` values.
        ///
        /// If both are `Some`, returns the greater value. If one is `None`, returns the other.
        /// If both are `None`, returns `None`.
        ///
        /// # Notes
        /// - This method relies on `PartialOrd`, which means it can handle types like `f64`.
        /// - For types like `f64`, be aware that `NaN` is not comparable. If either value is `NaN`,
        ///   it may result in unexpected behavior.
        ///   Consider pre-processing your data if `NaN` values are possible.
        fn min_or(self, other: Option<T>) -> Option<T>;

        /// Returns the maximum of two `Option` values if both are `Some`.
        ///
        /// If one is `None`, returns `None`. If both are `Some`, returns the greater value.
        ///
        /// # Notes
        /// - This method relies on `PartialOrd`, which means it can handle types like `f64`.
        /// - For types like `f64`, be aware that `NaN` is not comparable. If either value is `NaN`,
        ///   it may result in unexpected behavior.
        ///   Consider pre-processing your data if `NaN` values are possible.
        fn max_and(self, other: Option<T>) -> Option<T>;
    }

    impl<T: PartialOrd> OptionExt<T> for Option<T> {
        fn min_or(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => {
                    if x < y {
                        Some(x)
                    } else {
                        Some(y)
                    }
                }
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            }
        }

        fn max_and(self, other: Option<T>) -> Option<T> {
            match (self, other) {
                (Some(x), Some(y)) => {
                    if x > y {
                        Some(x)
                    } else {
                        Some(y)
                    }
                }
                _ => None,
            }
        }
    }
}
