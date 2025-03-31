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
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    let mut acc = mylib::Cumsum2D::new(101, 101);
    for (a, b) in ab {
        acc.add(a, b, 1);
    }
    acc.build();

    let mut ans = 0;

    for i in 1..=(100 - k) {
        for j in 1..=(100 - k) {
            ans = ans.max(acc.sum(i, j, i + k + 1, j + k + 1));
        }
    }

    println!("{}", ans);
}

pub mod mylib {
    /// A 2D cumulative sum structure for efficient range sum queries.
    #[derive(Debug, Clone)]
    pub struct Cumsum2D<T> {
        data: Vec<Vec<T>>,
        cum: Vec<Vec<T>>,
        h: usize,
        w: usize,
    }

    impl<T> Cumsum2D<T>
    where
        T: Default
            + Copy
            + std::ops::AddAssign
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>,
    {
        /// Creates a new `Cumsum2D` with the given height and width.
        /// The initial values are set to `T::default()`.
        pub fn new(h: usize, w: usize) -> Self {
            Self {
                data: vec![vec![T::default(); w]; h],
                cum: vec![vec![T::default(); w + 1]; h + 1],
                h,
                w,
            }
        }

        /// Creates a `Cumsum2D` from a given 2D vector.
        /// The input vector is **consumed** and its ownership is transferred.
        ///
        /// # Panics (Debug mode only)
        /// - If the input `data` has rows of different lengths.
        pub fn from_vec(data: Vec<Vec<T>>) -> Self {
            let h = data.len();
            let w = if h > 0 { data[0].len() } else { 0 };

            debug_assert!(
                data.iter().all(|row| row.len() == w),
                "All rows must have the same length"
            );

            let cum = vec![vec![T::default(); w + 1]; h + 1];

            Self { data, cum, h, w }
        }

        /// Sets a value at a specific position `(x, y)`.
        pub fn set(&mut self, x: usize, y: usize, value: T) {
            self.data[x][y] = value;
        }

        /// Adds a value to the existing value at `(x, y)`.
        pub fn add(&mut self, x: usize, y: usize, value: T) {
            self.data[x][y] += value;
        }

        /// Returns the value at `(x, y)`.
        pub fn get(&self, x: usize, y: usize) -> T {
            self.data[x][y]
        }

        /// Builds the cumulative sum table.
        pub fn build(&mut self) {
            for i in 0..self.h {
                for j in 0..self.w {
                    self.cum[i + 1][j + 1] =
                        self.data[i][j] + self.cum[i][j + 1] + self.cum[i + 1][j] - self.cum[i][j];
                }
            }
        }

        /// Computes the sum of values in the rectangular region `(x1, y1)` to `(x2, y2)`, exclusive.
        ///
        /// # Returns
        /// - The cumulative sum in the given range.
        ///
        /// # Complexity
        /// - `O(1)`
        pub fn sum(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> T {
            self.cum[x1][y1] + self.cum[x2][y2] - self.cum[x1][y2] - self.cum[x2][y1]
        }
    }
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, k: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = 0;

    for a in 1..100 {
        for b in 1..100 {
            ans = ans.max(
                (0..n)
                    .filter(|&i| {
                        a <= ab[i].0 && ab[i].0 <= a + k && b <= ab[i].1 && ab[i].1 <= b + k
                    })
                    .count(),
            );
        }
    }

    println!("{}", ans);
}
