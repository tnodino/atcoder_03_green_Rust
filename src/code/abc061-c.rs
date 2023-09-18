// https://atcoder.jp/contests/abc061/tasks/abc061_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            (a, b): (usize, usize),
        }
        vec.push((a, b));
    }
    vec.sort();
    let mut cnt = 0;
    for i in 0..N {
        cnt += vec[i].1;
        if cnt >= K {
            println!("{}", vec[i].0);
            return;
        }
    }
}