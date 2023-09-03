// https://atcoder.jp/contests/abc124/tasks/abc124_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut vec = Vec::new();
    let mut cnt = 0;
    let mut flg = 1;
    for i in 0..N {
        let x = (S[i] as usize) - ('0' as usize);
        if flg != x {
            vec.push((flg, cnt));
            cnt = 0;
            flg = x;
        }
        cnt += 1;
    }
    vec.push((flg, cnt));
    let mut l = 0;
    let mut s = 0;
    let mut cnt = 0;
    let mut ans = 0;
    for r in 0..vec.len() {
        if vec[r].0 == 0 {
            cnt += 1;
        }
        s += vec[r].1;
        while l <= r && K < cnt {
            if vec[l].0 == 0 {
                cnt -= 1;
            }
            s -= vec[l].1;
            l += 1;
        }
        ans = max(ans, s);
    }
    println!("{}", ans);
}