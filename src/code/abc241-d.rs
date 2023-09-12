// https://atcoder.jp/contests/abc241/tasks/abc241_d

use proconio::input;
use proconio::fastout;
use std::collections::BTreeSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut set = BTreeSet::new();
    for i in 0..Q {
        input! {
            q: usize,
        }
        match q {
            1 => {
                input! {
                    x: isize,
                }
                set.insert((x, i));
            },
            2 => {
                input! {
                    (x, k): (isize, usize),
                }
                println!("{}", set.range(..(x, i)).rev().nth(k-1).map_or(-1, |x| x.0));
            },
            3 => {
                input! {
                    (x, k): (isize, usize),
                }
                println!("{}", set.range((x, 0)..).nth(k-1).map_or(-1, |x| x.0));
            },
            _ => {},
        }
    }
}