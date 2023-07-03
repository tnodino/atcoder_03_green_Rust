// https://atcoder.jp/contests/abc128/tasks/abc128_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut k = Vec::new();
    let mut s = Vec::new();
    for _ in 0..M {
        input! {
            a: usize,
            b: [usize; a],
        }
        k.push(a);
        s.push(b);
    }
    input! {
        p: [usize; M],
    }
    let mut ans = 0;
    for bit in 0..1<<N {
        let mut flg = vec![0; N];
        for i in 0..N {
            if bit & (1 << i) > 0 {
                flg[i] = 1;
            }
        }
        let mut q = vec![0; M];
        for i in 0..M {
            for j in 0..k[i] {
                q[i] ^= flg[s[i][j]-1];
            }
        }
        if (0..M).all(|i| p[i] == q[i]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}