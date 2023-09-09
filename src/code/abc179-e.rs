// https://atcoder.jp/contests/abc179/tasks/abc179_e

use proconio::input;
use proconio::fastout;

fn f(x: usize, m: usize) -> usize {
    return x * x % m;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X, M): (usize, usize, usize),
    }
    let mut nxt = vec![0; M];
    for i in 0..M {
        nxt[i] = f(i, M);
    }
    let mut flg = vec![false; M];
    let mut pos = X;
    let mut ans = 0;
    for i in 1..=N {
        if flg[pos] {
            let mut vec = vec![pos];
            let st = pos;
            pos = nxt[pos];
            while st != pos {
                vec.push(pos);
                pos = nxt[pos];
            }
            let M = vec.len();
            let K = N - i;
            ans += vec.iter().sum::<usize>() * (K / M) + vec[..=(K % M)].iter().sum::<usize>();
            break;
        }
        flg[pos] = true;
        ans += pos;
        pos = nxt[pos];
    }
    println!("{}", ans);
}