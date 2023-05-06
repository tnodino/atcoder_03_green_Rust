// https://atcoder.jp/contests/abc050/tasks/arc066_a

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort();
    let mut x;
    let mut cnt;
    if N % 2 == 0 {
        x = 1;
        cnt = 0;
    }
    else {
        x = 0;
        cnt = 1;
    }
    for i in 0..N {
        if A[i] != x {
            println!("0");
            return;
        }
        cnt += 1;
        if cnt == 2 {
            cnt = 0;
            x += 2;
        }
    }
    let mut ans = 1;
    for _ in 0..N/2 {
        ans *= 2;
        ans %= MOD;
    }
    println!("{}", ans);
}