// https://atcoder.jp/contests/abc084/tasks/abc084_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let M = 100_000;
    let mut sieve = vec![true; M+1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=M {
        if sieve[i] {
            for j in (i..=M).step_by(i) {
                sieve[j] = false;
            }
            sieve[i] = true;
        }
    }
    let mut cnt = vec![0; M+1];
    for i in (3..=M).step_by(2) {
        if sieve[i] && sieve[(i+1)/2] {
            cnt[i] = 1;
        }
    }
    for i in 2..=M {
        cnt[i] += cnt[i-1];
    }
    for _ in 0..Q {
        input! {
            l: usize,
            r: usize,
        }
        println!("{}", cnt[r] - cnt[l-1]);
    }
}