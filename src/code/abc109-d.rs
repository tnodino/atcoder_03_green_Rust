// https://atcoder.jp/contests/abc109/tasks/abc109_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        mut a: [[usize; W]; H],
    }
    let mut ans = Vec::new();
    for i in 0..H {
        for j in 0..W-1 {
            if a[i][j] % 2 != 0 {
                a[i][j+1] += 1;
                ans.push((i + 1, j + 1, i + 1, j + 2));
            }
        }
    }
    for i in 0..H-1 {
        if a[i][W-1] % 2 != 0 {
            a[i+1][W-1] += 1;
            ans.push((i + 1, W, i + 2, W));
        }
    }
    let N = ans.len();
    println!("{}", N);
    for i in 0..N {
        println!("{} {} {} {}", ans[i].0, ans[i].1, ans[i].2, ans[i].3);
    }
}