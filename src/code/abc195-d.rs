// https://atcoder.jp/contests/abc195/tasks/abc195_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, Q): (usize, usize, usize),
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            (W, V): (usize, usize),
        }
        vec.push((V, W));
    }
    vec.sort_by(|a, b|
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        }
        else {
            b.1.cmp(&a.1)
        });
    input! {
        X: [usize; M],
    }
    for _ in 0..Q {
        input! {
            (L, R): (usize, usize),
        }
        let mut x = Vec::new();
        for i in 0..M {
            if i < L - 1 || R - 1 < i {
                x.push(X[i]);
            }
        }
        let m = x.len();
        x.sort();
        let mut ans = 0;
        for i in 0..N {
            for j in 0..m {
                if x[j] >= vec[i].1 {
                    x[j] = 0;
                    ans += vec[i].0;
                    break;
                }
            }
        }
        println!("{}", ans);
    }
}