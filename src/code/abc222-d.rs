// https://atcoder.jp/contests/abc222/tasks/abc222_d

use proconio::input;
use proconio::fastout;
use ac_library::ModInt998244353 as Mint;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
        b: [usize; N],
    }
    let M = 3000;
    let mut DP = vec![vec![Mint::new(0); M+1]; N+1];
    DP[0] = vec![1.into(); M+1];
    for i in 0..N {
        for j in a[i]..=b[i] {
            DP[i+1][j] = DP[i+1][j] + DP[i][j];
        }
        for j in 0..M {
            DP[i+1][j+1] = DP[i+1][j+1] + DP[i+1][j];
        }
    }
    println!("{}", DP[N][M]);
}