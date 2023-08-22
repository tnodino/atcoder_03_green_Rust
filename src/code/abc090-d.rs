// https://atcoder.jp/contests/abc090/tasks/arc091_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    if K == 0 {
        println!("{}", N * N);
        return;
    }
    let mut ans = 0;
    for b in K+1..=N {
        let x = (N + 1) / b;
        let y = (N + 1) % b;
        ans += x * (b - K);
        if y > K {
            ans += y - K;
        }
    }
    println!("{}", ans);
}