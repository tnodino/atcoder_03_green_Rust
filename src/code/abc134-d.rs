// https://atcoder.jp/contests/abc134/tasks/abc134_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut flg = vec![0; N];
    for i in (0..N).rev() {
        let mut x = 0;
        for j in (i..N).step_by(i+1) {
            x ^= flg[j];
        }
        flg[i] = a[i] ^ x;
    }
    let mut ans = Vec::new();
    for i in 0..N {
        if flg[i] == 1 {
            ans.push(i+1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}