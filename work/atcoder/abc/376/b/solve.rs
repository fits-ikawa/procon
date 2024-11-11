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
        n: isize, q: usize,
    }

    let mut left = 0;
    let mut right = 1;
    let mut ans = 0;

    for _ in 0..q {
        input! {
            hand: char, num: Isize1,
        }
        let me = if hand == 'L' { left } else { right };
        let he = if hand == 'L' { right } else { left };

        // 時計回りでの移動距離を調べ、自分の方が近ければ（＝間に反対の手がなければ）採用
        // 相手の方が近ければ不採用（移動距離を isize::MAX とする）
        let me_cw = (num - me + n) % n;
        let he_cw = (num - he + n) % n;
        let cw = if me_cw <= he_cw { me_cw } else { isize::MAX };

        // 反時計回りでも同じことを調べる
        let me_ccw = (me - num + n) % n;
        let he_ccw = (he - num + n) % n;
        let ccw = if me_ccw <= he_ccw { me_ccw } else { isize::MAX };

        // 採用した方を答えに足す
        ans += cw.min(ccw);

        if hand == 'L' {
            left = num;
        } else {
            right = num;
        }
    }

    println!("{}", ans);
}
