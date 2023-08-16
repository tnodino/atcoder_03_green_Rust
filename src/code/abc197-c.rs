// https://atcoder.jp/contests/abc197/tasks/abc197_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 1<<60;
    for bit in 0..1<<N {
        let mut res = 0;
        let mut x = 0;
        for i in 0..N {
            if bit & (1 << i) > 0 {
                res ^= x;
                x = A[i];
            }
            else {
                x |= A[i];
            }
        }
        res ^= x;
        ans = min(ans, res);
    }
    println!("{}", ans);
}