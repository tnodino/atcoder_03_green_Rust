// https://atcoder.jp/contests/abc036/tasks/abc036_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut a = Vec::new();
    for i in 0..N {
        input! {
            v: usize,
        }
        a.push((v, i));
    }
    a.sort();
    let mut b = vec![0; N];
    for i in 1..N {
        if a[i-1].0 == a[i].0 {
            b[i] = b[i-1];
        }
        else {
            b[i] = b[i-1] + 1;
        }
    }
    for i in 0..N {
        a[i].0 = b[i];
    }
    a.sort_by(|a, b| a.1.cmp(&b.1));
    for i in 0..N {
        println!("{}", a[i].0);
    }
}