// https://atcoder.jp/contests/abc300/tasks/abc300_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 300_000;
    let mut sieve = vec![true; M+1];
    let mut p = Vec::new();
    for i in 2..=M {
        if sieve[i] {
            for j in (i..=M).step_by(i) {
                sieve[j] = false;
            }
            p.push(i);
        }
    }
    let M = p.len();
    let mut ans: usize = 0;
    for i in 0..M {
        for j in i+1..M {
            for k in j+1..M {
                let mut v = p[i] * p[i] * p[j];
                if v > N {
                    break;
                }
                v *= p[k];
                if v > N {
                    break;
                }
                v *= p[k];
                if v > N {
                    break;
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}