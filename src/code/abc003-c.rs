// https://atcoder.jp/contests/abc003/tasks/abc003_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut R: [f64; N],
    }
    R.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut ans = 0.;
    for i in (0..K).rev() {
        ans = (ans + R[i]) / 2.;
    }
    println!("{}", ans);
}