// https://atcoder.jp/contests/abc113/tasks/abc113_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        M: usize,
    }
    let mut vec = Vec::new();
    for i in 0..M {
        input! {
            P: usize,
            Y: usize,
        }
        vec.push((P, Y, i))
    }
    vec.sort_by(|a, b|
    if a.0 != b.0 {
        a.0.cmp(&b.0)
    }
    else {
        a.1.cmp(&b.1)
    });
    let mut ans = vec![(0, 0); M];
    let mut num = vec[0].0;
    let mut cnt = 0;
    for i in 0..M {
        if vec[i].0 == num {
            cnt += 1;
        }
        else {
            num = vec[i].0;
            cnt = 1;
        }
        ans[vec[i].2] = (num, cnt);
    }
    for i in 0..M {
        println!("{:06}{:06}", ans[i].0, ans[i].1);
    }
}