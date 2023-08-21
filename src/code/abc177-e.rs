// https://atcoder.jp/contests/abc177/tasks/abc177_e

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 1_000_000;
    let mut sieve = vec![0; M+1];
    for i in 2..=M {
        if sieve[i] == 0 {
            for j in (i..=M).step_by(i) {
                sieve[j] = i;
            }
        }
    }
    let mut g = 0;
    let mut cnt = vec![0; M+1];
    for i in 0..N {
        g = gcd(g, A[i]);
        let mut a = A[i];
        while a > 1 {
            let s = sieve[a];
            cnt[s] += 1;
            while a % s == 0 {
                a /= s;
            }
        }
    }
    if cnt.iter().filter(|&x| *x >= 2).count() == 0 {
        println!("pairwise coprime");
    }
    else if g == 1 {
        println!("setwise coprime");
    }
    else {
        println!("not coprime");
    }
}