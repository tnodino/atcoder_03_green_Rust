// https://atcoder.jp/contests/abc132/tasks/abc132_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let M = 2000;
    let mut pascal = vec![vec![0; M+1]; M+1];
    pascal[0][0] = 1;
    for i in 1..=M {
        for j in 0..=i {
            if j == 0 {
                pascal[i][j] = 1;
            }
            else {
                pascal[i][j] = pascal[i-1][j] + pascal[i-1][j-1];
                pascal[i][j] %= MOD;
            }
        }
    }
    for i in 1..=K {
        println!("{}", pascal[N+1-K][i] * pascal[K-1][i-1] % MOD);
    }
}