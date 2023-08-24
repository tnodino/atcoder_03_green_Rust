// https://atcoder.jp/contests/abc084/tasks/abc084_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut C = Vec::new();
    let mut S = Vec::new();
    let mut F = Vec::new();
    for _ in 0..N-1 {
        input! {
            c: usize,
            s: usize,
            f: usize,
        }
        C.push(c);
        S.push(s);
        F.push(f);
    }
    for i in 0..N {
        let mut res = 0;
        for j in i..N-1 {
            if res < S[j] {
                res = S[j];
            }
            else if res % F[j] > 0 {
                res = res + F[j] - res % F[j];
            }
            res += C[j];
        }
        println!("{}", res);
    }
}