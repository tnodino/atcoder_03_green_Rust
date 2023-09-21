// https://atcoder.jp/contests/abc230/tasks/abc230_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, D): (usize, usize),
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            (L, R): (usize, usize),
        }
        vec.push((L, R));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    let mut r = 0;
    let mut ans = 0;
    for i in 0..N {
        if r <= vec[i].0 {
            r = vec[i].1 + D;
            ans += 1;
        }
    }
    println!("{}", ans);
}