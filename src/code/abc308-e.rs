// https://atcoder.jp/contests/abc308/tasks/abc308_e

use proconio::input;
use proconio::fastout;

fn f(x: usize, y: usize, z: usize) -> usize {
    let mut flg = vec![true; 3];
    flg[x] = false;
    flg[y] = false;
    flg[z] = false;
    let mut ret = 3;
    for i in 0..3 {
        if flg[i] {
            ret = i;
            break;
        }
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut m = vec![vec![0; 3]; N+1];
    let mut x = vec![vec![0; 3]; N+1];
    for i in 0..N {
        for j in 0..3 {
            m[i+1][j] = m[i][j];
            x[i+1][j] = x[i][j];
        }
        if S[i] == 'M' {
            m[i+1][A[i]] += 1;
        }
        if S[i] == 'X' {
            x[i+1][A[i]] += 1;
        }
    }
    let mut ans = 0;
    for i in 1..N-1 {
        if S[i] == 'E' {
            for j in 0..3 {
                for k in 0..3 {
                    let mex = f(A[i], j, k);
                    ans += m[i][j] * (x[N][k] - x[i+1][k]) * mex;
                }
            }
        }
    }
    println!("{}", ans);
}