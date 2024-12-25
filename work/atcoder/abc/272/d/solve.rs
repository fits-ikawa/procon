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
        n: usize, m: isize,
    }

    let squares = (0..n as isize).map(|i| i * i).collect::<BTreeSet<_>>();
    let mut pairs = vec![];

    for &sq in &squares {
        if squares.contains(&(m - sq)) {
            pairs.push((sq.sqrt(), (m - sq).sqrt()));
        }
    }

    let moving = pairs
        .into_iter()
        .flat_map(|(a, b)| [(a, b), (-a, b), (-a, -b), (a, -b)])
        .unique()
        .collect_vec();

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![None; n + 1]; n + 1]; // コストを記録

    todo.push_back((1, 1));
    seen[1][1] = Some(0);

    while let Some((x, y)) = todo.pop_front() {
        for &(dx, dy) in &moving {
            let nx = x + dx;
            let ny = y + dy;
            if 1 <= nx
                && nx <= n as isize
                && 1 <= ny
                && ny <= n as isize
                && seen[nx as usize][ny as usize].is_none()
            {
                seen[nx as usize][ny as usize] = seen[x as usize][y as usize].map(|e| e + 1);
                todo.push_back((nx, ny));
            }
        }
    }

    let mut ans = vec![vec![0; n]; n];

    for i in 1..=n {
        for j in 1..=n {
            ans[i - 1][j - 1] = seen[i][j].unwrap_or(-1);
        }
    }

    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
}
