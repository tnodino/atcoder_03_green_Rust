// https://atcoder.jp/contests/abc312/tasks/abc312_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut DP = vec![vec![0; N+1]; N+1];
    DP[0][0] = 1;
    for i in 0..N {
        for j in 0..=N {
            if j < N && (S[i] == '(' || S[i] == '?') {
                DP[i+1][j+1] += DP[i][j];
                DP[i+1][j+1] %= MOD;
            }
            if 0 < j && (S[i] == ')' || S[i] == '?') {
                DP[i+1][j-1] += DP[i][j];
                DP[i+1][j-1] %= MOD;
            }
        }
    }
    println!("{}", DP[N][0]);
}