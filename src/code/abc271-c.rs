// https://atcoder.jp/contests/abc271/tasks/abc271_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut flg = vec![false; N+10];
    let mut cnt = 0;
    for i in 0..N {
        if a[i] > N || flg[a[i]] {
            cnt += 1;
        }
        else {
            flg[a[i]] = true;
        }
    }
    let mut l = 1;
    let mut r = N;
    loop {
        while flg[l] {
            l += 1;
        }
        while r > 0 && !flg[r] {
            r -= 1;
        }
        if cnt >= 2 {
            cnt -= 2;
            flg[l] = true;
        }
        else if l >= r {
            break;
        }
        else {
            cnt += 1;
            flg[r] = false;
        }
    }
    println!("{}", l - 1);
}