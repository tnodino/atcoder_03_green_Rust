// https://atcoder.jp/contests/abc221/tasks/abc221_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        vec.push((A, 1));
        vec.push((A + B, 2));
    }
    vec.sort();
    let mut l = 0;
    let mut cnt = 0;
    let mut ans = vec![0; N+1];
    for i in 0..N*2 {
        if vec[i].0 != l {
            ans[cnt] += vec[i].0 - l;
        }
        l = vec[i].0;
        match vec[i].1 {
            1 => cnt += 1,
            _ => cnt -= 1,
        }
    }
    println!("{}", ans[1..].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}