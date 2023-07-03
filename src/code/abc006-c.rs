// https://atcoder.jp/contests/abc006/tasks/abc006_3

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    for y in 0..=N {
        if y * 3 > M {
            break;
        }
        if (M - y * 3) % 2 == 1 {
            continue;
        }
        let n = N - y;
        let m = (M - y * 3) / 2;
        if n * 2 < m {
            continue;
        }
        let x = n * 2 - m;
        if n < x {
            continue;
        }
        let z = n - x;
        println!("{} {} {}", x, y, z);
        return;
    }
    println!("-1 -1 -1");
}