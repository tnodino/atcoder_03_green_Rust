// https://atcoder.jp/contests/abc112/tasks/abc112_c

use proconio::input;
use std::cmp::max;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut h = Vec::new();
    let mut idx = 0;
    for i in 0..N {
        input! {
            a: isize,
            b: isize,
            c: isize,
        }
        x.push(a);
        y.push(b);
        h.push(c);
        if c > 0 {
            idx = i;
        }
    }
    for Cx in 0..=100 {
        'cont: for Cy in 0..=100 {
            let H = h[idx] + (Cx - x[idx]).abs() + (Cy - y[idx]).abs();
            if 1 <= H {
                for i in 0..N {
                    if max(H - (Cx - x[i]).abs() - (Cy - y[i]).abs(), 0) != h[i] {
                        continue 'cont;
                    }
                }
                println!("{} {} {}", Cx, Cy, H);
                return;
            }
        }
    }
}