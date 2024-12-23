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
        _w: usize, n: usize,
        lr: [(usize, usize); n],
    }

    let mut height = btreemap! {}; // 高さが変化した点 -> 高さ のマップ
    height.insert(0, 0);

    for (l, r) in lr {
        let &hl = height.range(..=l).last().unwrap().1;
        let &hr = height.range(..=r).last().unwrap().1;

        let mut max_h = hl;
        let mut beneath = vec![];

        // 今回のブロックによって覆われる箇所と、その最大高さを取得
        for (&pos, &hp) in height.range(..=r).rev() {
            if pos < l {
                break;
            }
            max_h = max_h.max(hp);
            beneath.push(pos);
        }

        // 覆われた変化点は削除
        for pos in beneath {
            height.remove(&pos);
        }

        height.insert(l, max_h + 1);
        if !height.contains_key(&(r + 1)) {
            height.insert(r + 1, hr);
        }

        println!("{}", max_h + 1);
    }
}
