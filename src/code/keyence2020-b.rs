// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut bots = Vec::new();
    for _ in 0..N {
        input! {
            X: isize,
            L: isize,
        }
        bots.push((X - L, X + L));
    }
    bots.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ans = 1;
    let mut r = bots[0].1;
    for i in 1..N {
        if r <= bots[i].0 {
            ans += 1;
            r = bots[i].1;
        }
    }
    println!("{}", ans);
}