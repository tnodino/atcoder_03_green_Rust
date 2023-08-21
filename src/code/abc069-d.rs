// https://atcoder.jp/contests/abc069/tasks/arc080_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        a: [usize; N],
    }
    let mut ans = vec![vec![0; W]; H];
    let mut idx = 0;
    let mut cnt = 0;
    for i in 0..H {
        if i % 2 == 0 {
            for j in 0..W {
                ans[i][j] = idx + 1;
                cnt += 1;
                if a[idx] == cnt {
                    idx += 1;
                    cnt = 0;
                }
            }
        }
        else {
            for j in (0..W).rev() {
                ans[i][j] = idx + 1;
                cnt += 1;
                if a[idx] == cnt {
                    idx += 1;
                    cnt = 0;
                }
            }
        }
    }
    for i in 0..H {
        println!("{}", ans[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}