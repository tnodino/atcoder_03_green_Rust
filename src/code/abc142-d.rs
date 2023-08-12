// https://atcoder.jp/contests/abc142/tasks/abc142_d

use proconio::input;
use proconio::fastout;
use num::integer::gcd;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let mut g = gcd(A, B);
    let M = sqrt(A as f64) as usize + 1;
    let mut ans = 1;
    for i in 2..=M {
        if g % i == 0 {
            ans += 1;
            while g % i == 0 {
                g /= i;
            }
        }
    }
    if g > 1 {
        ans += 1;
    }
    println!("{}", ans);
}